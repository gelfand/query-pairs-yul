object "Contract" {
    code {
        datacopy(0, dataoffset("runtime"), datasize("runtime"))
        return(0, datasize("runtime"))
    }
    object "runtime" {
        code {
            if iszero(eq(0x0000000000000000000000000000000000000000, caller())) { revert(0, 0) }

            switch shr(0xf8, calldataload(0))
            case 0x00 {
                let factory := shr(0x60, calldataload(0x01))
                let allPairsLength 
                {
                    mstore(0, shl(0xe0, 0x574f2ba3))
                    if iszero(staticcall(gas(), factory, 0, 0x04, 0x04, 0x20)) { revert(0, 0) }
                    allPairsLength := mload(0x04)
                }

                let start_slot := 0x100
                for { let i := 0 } iszero(eq(i, allPairsLength)) { i := add(i, 0x01) } {
                    mstore(0x40, shl(0xe0, 0x1e3dd18b))
                    mstore(add(0x40, 0x04), i)
                    if iszero(staticcall(gas(), factory, 0x40, 0x24, start_slot, 0x20)) { revert(0, 0) }
                    start_slot := add(start_slot, 0x20)
                }
                let end_slot := sub(start_slot, 0x100)

                return(0x100, end_slot)
            }
            case 0x01 {
                let size := calldatasize()

                let start_slot := 0x40

                for { let i := 0x01 } iszero(eq(i, size)) { i := add(i, 0x20) } {
                    mstore(0x00, shl(0xe0, 0x0902f1ac))
                    let pair_id := calldataload(i)
                    if iszero(staticcall(gas(), pair_id, 0, 0x04, start_slot, 0x60)) { revert(0, 0) }
                    start_slot := add(start_slot, 0x60)
                }

                let end_slot := sub(start_slot, 0x40)

                return(0x40, end_slot)
            }
            default {
                revert(0, 0)
            }
        }
    }
}
