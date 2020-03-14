//! The `RemoveUnitStorage` pass removes `StorageLive` and `StorageDead` statements
//! which operates on locals of type `()`.

use crate::transform::{MirPass, MirSource};
use rustc::mir::*;
use rustc::ty::TyCtxt;
use smallvec::SmallVec;

pub struct RemoveUnitStorage;

impl<'tcx> MirPass<'tcx> for RemoveUnitStorage {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, _: MirSource<'tcx>, body: &mut BodyAndCache<'tcx>) {
        let unit_locals: SmallVec<[_; 32]> =
            body.local_decls.iter().map(|local| local.ty == tcx.types.unit).collect();

        for block in body.basic_blocks_mut() {
            for stmt in &mut block.statements {
                match &stmt.kind {
                    StatementKind::StorageLive(l) | StatementKind::StorageDead(l) => {
                        if unit_locals[l.as_usize()] {
                            stmt.make_nop();
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
