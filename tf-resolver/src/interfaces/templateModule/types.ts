// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import { Struct } from '@polkadot/types/codec';
import { Bytes } from '@polkadot/types/primitive';
import { H256 } from '@polkadot/types/interfaces/runtime';

/** @name KycObject */
export interface KycObject extends Struct {
  readonly provider: Bytes;
  readonly proof: H256;
}

export type PHANTOM_TEMPLATEMODULE = 'templateModule';
