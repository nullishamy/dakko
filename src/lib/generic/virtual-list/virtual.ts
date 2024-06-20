/**
 * virtual list core calculating center
 */

const DIRECTION_TYPE = {
	FRONT: 'FRONT', // scroll up or left
	BEHIND: 'BEHIND' // scroll down or right
};
const CALC_TYPE = {
	INIT: 'INIT',
	FIXED: 'FIXED',
	DYNAMIC: 'DYNAMIC'
};

const LEADING_BUFFER = 2;

export interface VirtualRange {
	start: number;
	end: number;
	padFront: number;
	padBehind: number;
}

type VirtualRangeUpdater = (range: VirtualRange) => void;

type VirtualParam<K> = {
	keeps: number;
	slotHeaderSize: number;
	buffer: number;
	estimateSize: number;
	uniqueIds: K[];

	[k: string]: unknown;
};

export class Virtual<K> {
	param: VirtualParam<K> | undefined;
	callUpdate: VirtualRangeUpdater | undefined;

	firstRangeTotalSize: number | undefined = 0;
	fixedSizeValue: number | undefined = 0;

	firstRangeAverageSize = 0;
	lastCalcIndex = 0;
	calcType = CALC_TYPE.INIT;
	offset = 0;
	direction = '';
	range: VirtualRange = { start: 0, end: 0, padFront: 0, padBehind: 0 };
	sizes: Map<K, number> = new Map();

	constructor(param: VirtualParam<K> | undefined, callUpdate: VirtualRangeUpdater) {
		this.param = param;
		this.callUpdate = callUpdate;

		this.init(param);
	}

	init(param: VirtualParam<K> | undefined) {
		// size data
		this.sizes = new Map();
		this.firstRangeTotalSize = 0;
		this.firstRangeAverageSize = 0;
		this.fixedSizeValue = 0;
		this.calcType = CALC_TYPE.INIT;

		// scroll data
		this.offset = 0;
		this.direction = '';

		// range data
		if (param) {
			this.checkRange(0, param.keeps - 1);
		}
	}

	destroy() {
		this.param = undefined;
		this.callUpdate = undefined;
		this.init(undefined);
	}

	// return current render range
	getRange(): VirtualRange {
		return {
			...this.range
		};
	}

	isBehind() {
		return this.direction === DIRECTION_TYPE.BEHIND;
	}

	isFront() {
		return this.direction === DIRECTION_TYPE.FRONT;
	}

	// return start index offset
	getOffset(start: number): number {
		return (start < 1 ? 0 : this.getIndexOffset(start)) + (this.param?.slotHeaderSize ?? 0);
	}

	getParam(): VirtualParam<K> {
		const param = this.param;
		if (!param) {
			throw TypeError('expected param to be set');
		}

		return param;
	}

	updateParam<K2 extends keyof VirtualParam<K>, V = VirtualParam<K>[K2]>(key: K2, value: V) {
		if (this.param && key in this.param) {
			// if uniqueIds change, find out deleted id and remove from size map
			if (key === 'uniqueIds') {
				const uniqueIds = value as unknown as VirtualParam<K>['uniqueIds'];
				this.sizes.forEach((v, index) => {
					if (!uniqueIds.includes(index)) {
						this.sizes.delete(index);
					}
				});
			}

			this.param[key] = value;
		}
	}

	// save each size map by id
	saveSize(id: K, size: number) {
		this.sizes.set(id, size);

		// we assume size type is fixed at the beginning and remember first size value
		// if there is no size value different from this at next coming saving
		// we think it's a fixed size list, otherwise is dynamic size list
		if (this.calcType === CALC_TYPE.INIT) {
			this.fixedSizeValue = size;
			this.calcType = CALC_TYPE.FIXED;
		} else if (this.calcType === CALC_TYPE.FIXED && this.fixedSizeValue !== size) {
			this.calcType = CALC_TYPE.DYNAMIC;
			// it's no use at all
			delete this.fixedSizeValue;
		}

		// calculate the average size only in the first range
		if (this.calcType !== CALC_TYPE.FIXED && typeof this.firstRangeTotalSize !== 'undefined') {
			const param = this.getParam();
			if (this.sizes.size < Math.min(param.keeps, param.uniqueIds.length)) {
				this.firstRangeTotalSize = [...this.sizes.values()].reduce((acc, val) => acc + val, 0);
				this.firstRangeAverageSize = Math.round(this.firstRangeTotalSize / this.sizes.size);
			} else {
				// it's done using
				delete this.firstRangeTotalSize;
			}
		}
	}

	// in some special situation (e.g. length change) we need to update in a row
	// try going to render next range by a leading buffer according to current direction
	handleDataSourcesChange() {
		let start = this.range.start;

		if (this.isFront()) {
			start = start - LEADING_BUFFER;
		} else if (this.isBehind()) {
			start = start + LEADING_BUFFER;
		}

		start = Math.max(start, 0);

		this.updateRange(this.range.start, this.getEndByStart(start));
	}

	// when slot size change, we also need force update
	handleSlotSizeChange() {
		this.handleDataSourcesChange();
	}

	// calculating range on scroll
	handleScroll(offset: number) {
		this.direction = offset < this.offset ? DIRECTION_TYPE.FRONT : DIRECTION_TYPE.BEHIND;
		this.offset = offset;

		if (!this.param) {
			return;
		}

		if (this.direction === DIRECTION_TYPE.FRONT) {
			this.handleFront();
		} else if (this.direction === DIRECTION_TYPE.BEHIND) {
			this.handleBehind();
		}
	}

	// ----------- public method end -----------

	handleFront() {
		const overs = this.getScrollOvers();
		// should not change range if start doesn't exceed overs
		if (overs > this.range.start) {
			return;
		}

		// move up start by a buffer length, and make sure its safety
		const param = this.getParam();
		const start = Math.max(overs - param.buffer, 0);
		this.checkRange(start, this.getEndByStart(start));
	}

	handleBehind() {
		const overs = this.getScrollOvers();
		const param = this.getParam();
		// range should not change if scroll overs within buffer
		if (overs < this.range.start + param.buffer) {
			return;
		}

		this.checkRange(overs, this.getEndByStart(overs));
	}

	// return the pass overs according to current scroll offset
	getScrollOvers() {
		// if slot header exist, we need subtract its size
		const param = this.getParam();
		const offset = this.offset - param.slotHeaderSize;
		if (offset <= 0) {
			return 0;
		}

		// if is fixed type, that can be easily
		if (this.isFixedType()) {
			const fixedSize = this.fixedSizeValue;
			if (fixedSize === undefined) {
				throw new TypeError('expected fixedSize to be set for fixed type');
			}

			return Math.floor(offset / fixedSize);
		}

		let low = 0;
		let middle = 0;
		let middleOffset = 0;
		let high = param.uniqueIds.length;

		while (low <= high) {
			// this.__bsearchCalls++
			middle = low + Math.floor((high - low) / 2);
			middleOffset = this.getIndexOffset(middle);

			if (middleOffset === offset) {
				return middle;
			} else if (middleOffset < offset) {
				low = middle + 1;
			} else if (middleOffset > offset) {
				high = middle - 1;
			}
		}

		return low > 0 ? --low : 0;
	}

	// return a scroll offset from given index, can efficiency be improved more here?
	// although the call frequency is very high, its only a superposition of numbers
	getIndexOffset(givenIndex: number | undefined) {
		if (!givenIndex) {
			return 0;
		}

		let offset = 0;
		let indexSize: number | undefined = 0;
		for (let index = 0; index < givenIndex; index++) {
			// this.__getIndexOffsetCalls++
			const param = this.getParam();
			indexSize = this.sizes.get(param.uniqueIds[index]);
			offset = offset + (indexSize ?? this.getEstimateSize() ?? 0);
		}

		// remember last calculate index
		this.lastCalcIndex = Math.max(this.lastCalcIndex, givenIndex - 1);
		this.lastCalcIndex = Math.min(this.lastCalcIndex, this.getLastIndex());

		return offset;
	}

	// is fixed size type
	isFixedType() {
		return this.calcType === CALC_TYPE.FIXED;
	}

	// return the real last index
	getLastIndex() {
		const param = this.getParam();
		return param.uniqueIds.length - 1;
	}

	// in some conditions range is broke, we need correct it
	// and then decide whether need update to next range
	checkRange(start: number, end: number) {
		const param = this.getParam();
		const keeps = param.keeps;
		const total = param.uniqueIds.length;

		// data less than keeps, render all
		if (total <= keeps) {
			start = 0;
			end = this.getLastIndex();
		} else if (end - start < keeps - 1) {
			// if range length is less than keeps, correct it base on end
			start = end - keeps + 1;
		}

		if (this.range.start !== start) {
			this.updateRange(start, end);
		}
	}

	// setting to a new range and rerender
	updateRange(start: number, end: number) {
		this.range.start = start;
		this.range.end = end;
		this.range.padFront = this.getPadFront();
		this.range.padBehind = this.getPadBehind();
		this.callUpdate?.(this.getRange());
	}

	// return end base on start
	getEndByStart(start: number) {
		const param = this.getParam();
		const theoryEnd = start + param.keeps - 1;
		const truelyEnd = Math.min(theoryEnd, this.getLastIndex());
		return truelyEnd;
	}

	// return total front offset
	getPadFront() {
		if (this.isFixedType()) {
			const fixedSize = this.fixedSizeValue;
			if (fixedSize === undefined) {
				throw new TypeError('expected fixedSize to be set for fixed type');
			}

			return fixedSize * this.range.start;
		} else {
			return this.getIndexOffset(this.range.start);
		}
	}

	// return total behind offset
	getPadBehind() {
		const end = this.range.end;
		const lastIndex = this.getLastIndex();

		if (this.isFixedType()) {
			const fixedSize = this.fixedSizeValue;
			if (fixedSize === undefined) {
				throw new TypeError('expected fixedSize to be set for fixed type');
			}

			return (lastIndex - end) * fixedSize;
		}

		// if it's all calculated, return the exactly offset
		console.log(
			'lastCalcIndex',
			this.lastCalcIndex,
			'lastIndex',
			lastIndex,
			'end',
			end,
			'estimate',
			this.getEstimateSize()
		);

		if (this.lastCalcIndex === lastIndex) {
			return this.getIndexOffset(lastIndex) - this.getIndexOffset(end);
		} else {
			// if not, use a estimated value
			console.log('li-end', lastIndex - end, 'est', this.getEstimateSize());
			return 0
		}
	}

	// get the item estimate size
	getEstimateSize() {
		const param = this.getParam();

		if (this.isFixedType()) {
			const value = this.fixedSizeValue;
			if (value === undefined) {
				throw new TypeError('expected value to be set');
			}
			return value;
		}

		return this.firstRangeAverageSize || param.estimateSize;
	}
}

export function isBrowser() {
	return typeof document !== 'undefined';
}
