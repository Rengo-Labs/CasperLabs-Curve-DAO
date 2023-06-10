use crate::alloc::string::ToString;
use crate::data::{
    self, get_package_hash, ChangesSum, ChangesWeight, GaugeTypeNames, GaugeTypes_, Gauges,
    LastUserVote, Point, PointsSum, PointsTotal, PointsTypeWeight, PointsWeight, TimeSum,
    TimeTypeWeight, TimeWeight, VoteUserPower, VoteUserSlopes, VotedSlope, MULTIPLIER, WEEK,
    WEIGHT_VOTE_DELAY,
};
use alloc::collections::BTreeMap;
use alloc::string::String;
use casper_contract::contract_api::storage;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{runtime_args, ApiError, ContractPackageHash, Key, RuntimeArgs, U128, U256};
use casperlabs_contract_utils::{ContractContext, ContractStorage};
use common::{errors::*, utils::*};

pub enum GAUGECONLTROLLEREvent {
    Minted {
        recipient: Key,
        gauge: Key,
        minted: U256,
    },
    CommitOwnership {
        admin: Key,
    },
    ApplyOwnership {
        admin: Key,
    },
    NewTypeWeight {
        type_id: i128,
        time: U256,
        weight: U256,
        total_weight: U256,
    },
    NewGaugeWeight {
        gauge_address: Key,
        time: U256,
        weight: U256,
        total_weight: U256,
    },
    AddType {
        name: String,
        type_id: i128,
    },
    VoteForGauge {
        time: U256,
        user: Key,
        gauge_addr: Key,
        weight: U256,
    },
    NewGauge {
        addr: Key,
        gauge_type: i128,
        weight: U256,
    },
}

impl GAUGECONLTROLLEREvent {
    #[inline(always)]
    pub fn type_name(&self) -> String {
        match self {
            GAUGECONLTROLLEREvent::Minted {
                recipient: _,
                gauge: _,
                minted: _,
            } => "minted",
            GAUGECONLTROLLEREvent::CommitOwnership { admin: _ } => "CommitOwnership",
            GAUGECONLTROLLEREvent::ApplyOwnership { admin: _ } => "ApplyOwnership",
            GAUGECONLTROLLEREvent::NewTypeWeight {
                type_id: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewTypeWeight",
            GAUGECONLTROLLEREvent::NewGaugeWeight {
                gauge_address: _,
                time: _,
                weight: _,
                total_weight: _,
            } => "NewGaugeWeight",
            GAUGECONLTROLLEREvent::AddType {
                name: _,
                type_id: _,
            } => "AddType",
            GAUGECONLTROLLEREvent::VoteForGauge {
                time: _,
                user: _,
                gauge_addr: _,
                weight: _,
            } => "VoteForGauge",
            GAUGECONLTROLLEREvent::NewGauge {
                addr: _,
                gauge_type: _,
                weight: _,
            } => "NewGauge",
        }
        .to_string()
    }
}

pub trait GAUGECONLTROLLER<Storage: ContractStorage>: ContractContext<Storage> {
    /// @notice Contract constructor
    /// @param _token `ERC20CRV` contract address
    /// @param _voting_escrow `VotingEscrow` contract address
    fn init(
        &mut self,
        token: Key,
        voting_escrow: Key,
        contract_hash: Key,
        package_hash: ContractPackageHash,
    ) {
        let address_0: Key = zero_address();
        if token == address_0 {
            //Gauge Controller Address Zero 1
            runtime::revert(Error::GaugeControllerAddressZero1);
        }
        if voting_escrow == address_0 {
            //Gauge Controller Address Zero 2
            runtime::revert(Error::GaugeControllerAddressZero2);
        }
        data::set_token(token);
        data::set_voting_escrow(voting_escrow);
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_admin(self.get_caller());
        data::set_time_total(
            U256::from(u64::from(runtime::get_blocktime()))
                .checked_div(WEEK)
                .unwrap_or_revert_with(Error::GaugeControllerDivide1)
                .checked_mul(WEEK)
                .unwrap_or_revert_with(Error::GaugeControllerMultiply1),
        );
        GaugeTypeNames::init();
        GaugeTypes_::init();
        VoteUserSlopes::init();
        VoteUserPower::init();
        LastUserVote::init();
        PointsWeight::init();
        ChangesWeight::init();
        TimeWeight::init();
        Gauges::init();
        TimeSum::init();
        PointsSum::init();
        ChangesSum::init();
        PointsTotal::init();
        PointsTypeWeight::init();
        TimeTypeWeight::init();
    }

    #[inline(always)]
    fn commit_transfer_ownership(&mut self, addr: Key) {
        if self.get_caller() != self.admin() {
            //Gauge Controller Only Admin
            runtime::revert(Error::GaugeControllerOnlyAdmin1);
        }
        data::set_future_admin(addr);
        self.emit(&GAUGECONLTROLLEREvent::CommitOwnership { admin: addr });
    }
    #[inline(always)]
    fn apply_transfer_ownership(&mut self) {
        if self.get_caller() != self.admin() {
            //Gauge Controller Only Admin
            runtime::revert(Error::GaugeControllerOnlyAdmin2);
        }
        let _admin = self.future_admin();
        if _admin == zero_address() || _admin == account_zero_address() {
            //Gauge Controller Admin Not Set
            runtime::revert(Error::GaugeControllerAdminNotSet);
        }
        data::set_admin(_admin);
        self.emit(&GAUGECONLTROLLEREvent::ApplyOwnership { admin: _admin });
    }

    #[inline(always)]
    fn gauge_types(&mut self, _addr: Key) -> i128 {
        let gauge_type = self.gauge_types_(_addr);
        if gauge_type == 0 {
            //Gauge Controller Gauge Type Is Zero
            runtime::revert(Error::GaugeControllerGaugeTypeIsZero);
        }
        gauge_type
            .checked_sub(1)
            .unwrap_or_revert_with(Error::GaugeControllerUnderFlow1)
    }

    #[inline(always)]
    fn checkpoint(&mut self) {
        self._get_total();
    }

    #[inline(always)]
    fn checkpoint_gauge(&mut self, addr: Key) {
        self._get_weight(addr);
        self._get_total();
    }

    /// @notice Fill historic gauge weights week-over-week for missed checkins
    ///         and return the total for the future week
    /// @param gauge_addr Address of the gauge
    /// @return Gauge weight

    #[inline(always)]
    fn _get_weight(&mut self, gauge_addr: Key) -> U256 {
        let mut t: U256 = self.time_weight(gauge_addr);
        if t > U256::from(0) {
            let mut pt: Point = self.points_weight(gauge_addr, t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t
                    .checked_add(WEEK)
                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow1);

                let d_bias: U256 = pt
                    .slope
                    .checked_mul(data::WEEK)
                    .unwrap_or_revert_with(Error::GaugeControllerMultiply2);
                if pt.bias > d_bias {
                    pt.bias = pt
                        .bias
                        .checked_sub(d_bias)
                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow2);
                    let d_slope: U256 = self.changes_weight(gauge_addr, t);
                    pt.slope = pt
                        .slope
                        .checked_sub(d_slope)
                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow3);
                } else {
                    pt.bias = 0.into();
                    pt.slope = 0.into();
                }
                PointsWeight::instance().set(&gauge_addr, &t, pt);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TimeWeight::instance().set(&gauge_addr, t);
                }
            }
            pt.bias
        } else {
            U256::from(0)
        }
    }

    /// @notice Fill historic total weights week-over-week for missed checkins
    ///         and return the total for the future week
    /// @return Total weight
    #[inline(always)]
    fn _get_total(&mut self) -> U256 {
        let mut t: U256 = self.time_total();
        let mut _n_gauge_types: i128 = self.n_gauge_types();
        if t > U256::from(u64::from(runtime::get_blocktime())) {
            // If we have already checkpointed - still need to change the value
            t = t
                .checked_sub(WEEK)
                .unwrap_or_revert_with(Error::GaugeControllerUnderFlow4);
        }
        let mut pt: U256 = self.points_total(t);
        for gauge_type in 0..(100) {
            if gauge_type == _n_gauge_types {
                break;
            }
            self._get_sum(gauge_type);
            self._get_type_weight(gauge_type);
        }
        for _ in 0..(500) {
            if t > U256::from(u64::from(runtime::get_blocktime())) {
                break;
            }
            t = t
                .checked_add(WEEK)
                .unwrap_or_revert_with(Error::GaugeControllerOverFlow2);
            pt = U256::from(0);
            for gauge_type in 0..(100) {
                if gauge_type == _n_gauge_types {
                    break;
                }
                let type_sum: U256 = self.points_sum(gauge_type, t).bias;
                let type_weight: U256 = self.points_type_weight(gauge_type, t);
                pt = pt
                    .checked_add(
                        type_sum
                            .checked_mul(type_weight)
                            .unwrap_or_revert_with(Error::GaugeControllerMultiply3),
                    )
                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow3);
            }
            PointsTotal::instance().set(&t, pt);
            if t > U256::from(u64::from(runtime::get_blocktime())) {
                data::set_time_total(t);
            }
        }
        pt
    }

    /// @notice Fill sum of gauge weights for the same type week-over-week for
    ///         missed checkins and return the sum for the future week
    /// @param gauge_type Gauge type id
    /// @return Sum of weights
    #[inline(always)]
    fn _get_sum(&mut self, gauge_type: i128) -> U256 {
        let mut t: U256 = self.time_sum(U256::from(gauge_type));
        if t > U256::from(0) {
            let mut pt: Point = self.points_sum(gauge_type, t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t
                    .checked_add(WEEK)
                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow4);
                let d_bias: U256 = pt
                    .slope
                    .checked_mul(WEEK)
                    .unwrap_or_revert_with(Error::GaugeControllerMultiply4);
                if pt.bias > d_bias {
                    pt.bias = pt
                        .bias
                        .checked_sub(d_bias)
                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow5);
                    let d_slope: U256 = self.changes_sum(gauge_type, t);
                    pt.slope = d_slope;
                } else {
                    pt.bias = U256::from(0);
                    pt.slope = U256::from(0);
                }
                PointsSum::instance().set(&gauge_type, &t, pt);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TimeSum::instance().set(&U256::from(gauge_type), t)
                }
            }
            pt.bias
        } else {
            U256::from(0)
        }
    }

    /// @notice Fill historic type weights week-over-week for missed checkins
    ///         and return the type weight for the future week
    /// @param gauge_type Gauge type id
    /// @return Type weight
    #[inline(always)]
    fn _get_type_weight(&mut self, gauge_type: i128) -> U256 {
        let mut t: U256 = self.time_type_weight(U256::from(gauge_type));
        if t > U256::from(0) {
            let w: U256 = self.points_type_weight(gauge_type, t);
            for _ in 0..(500) {
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    break;
                }
                t = t
                    .checked_add(WEEK)
                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow5);
                PointsTypeWeight::instance().set(&gauge_type, &t, w);
                if t > U256::from(u64::from(runtime::get_blocktime())) {
                    TimeTypeWeight::instance().set(&U256::from(gauge_type), t)
                }
            }
            w
        } else {
            U256::from(0)
        }
    }

    /// @notice Get Gauge relative weight (not more than 1.0) normalized to 1e9
    ///         (e.g. 1.0 == 1e9). Inflation which will be received by it is
    ///         inflation_rate * relative_weight / 1e9
    /// @param addr Gauge address
    /// @param time Relative weight at the specified timestamp in the past or present
    /// @return Value of relative weight normalized to 1e9

    #[inline(always)]
    fn _gauge_relative_weight(&mut self, addr: Key, time: U256) -> U256 {
        let t: U256 = time
            .checked_div(WEEK)
            .unwrap_or_revert_with(Error::GaugeControllerDivide2)
            .checked_mul(WEEK)
            .unwrap_or_revert_with(Error::GaugeControllerMultiply5);
        let _total_weight = self.points_total(t);
        if _total_weight > U256::from(0) {
            let gauge_type: i128 = self
                .gauge_types_(addr)
                .checked_sub(1.into())
                .unwrap_or_revert_with(Error::GaugeControllerUnderFlow24);
            let _type_weight: U256 = self.points_type_weight(gauge_type, t);
            let _gauge_weight: U256 = self.points_weight(addr, t).bias;
            MULTIPLIER
                .checked_mul(_type_weight)
                .unwrap_or_revert_with(Error::GaugeControllerMultiply6)
                .checked_mul(_gauge_weight)
                .unwrap_or_revert_with(Error::GaugeControllerMultiply7)
                .checked_div(_total_weight)
                .unwrap_or_revert_with(Error::GaugeControllerDivide3)
        } else {
            U256::from(0)
        }
    }

    /// @notice Change type weight
    /// @param type_id Type id
    /// @param weight New type weight
    #[inline(always)]
    fn _change_type_weight(&mut self, type_id: i128, weight: U256) {
        let old_sum: U256 = self._get_sum(type_id);
        let _total_weight: U256 = self._get_total();
        let next_time: U256 = (U256::from(u64::from(runtime::get_blocktime()))
            .checked_add(WEEK)
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow6))
        .checked_div(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerDivide4)
        .checked_mul(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerMultiply8);

        let _total_weight = _total_weight
            .checked_add(
                (old_sum
                    .checked_mul(weight)
                    .unwrap_or_revert_with(Error::GaugeControllerMultiply9))
                .checked_sub(
                    old_sum
                        .checked_mul(self._get_type_weight(type_id))
                        .unwrap_or_revert_with(Error::GaugeControllerMultiply10),
                )
                .unwrap_or_revert_with(Error::GaugeControllerUnderFlow6),
            )
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow7);

        PointsTotal::instance().set(&next_time, _total_weight);
        PointsTypeWeight::instance().set(&type_id, &next_time, weight);
        data::set_time_total(next_time);
        TimeTypeWeight::instance().set(&U256::from(type_id), next_time);
        self.emit(&GAUGECONLTROLLEREvent::NewTypeWeight {
            type_id,
            time: next_time,
            weight,
            total_weight: _total_weight,
        });
    }

    /// Change gauge weight
    /// Only needed when testing in reality
    #[inline(always)]
    fn _change_gauge_weight(&mut self, addr: Key, weight: U256) {
        let gauge_type: i128 = self
            .gauge_types_(addr)
            .checked_sub(1)
            .unwrap_or_revert_with(Error::GaugeControllerUnderFlow7);
        let old_gauge_weight: U256 = self._get_weight(addr);
        let type_weight: U256 = self._get_type_weight(gauge_type);
        let old_sum: U256 = self._get_sum(gauge_type);
        let _total_weight: U256 = self._get_total();
        let next_time: U256 = (U256::from(u64::from(runtime::get_blocktime()))
            .checked_add(WEEK)
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow8))
        .checked_div(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerDivide5)
        .checked_mul(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerMultiply11);
        let mut points_wight = self.points_weight(addr, next_time);
        points_wight.bias = weight;
        PointsWeight::instance().set(&addr, &next_time, points_wight);
        TimeWeight::instance().set(&addr, next_time);
        let new_sum: U256 = old_sum
            .checked_add(
                weight
                    .checked_sub(old_gauge_weight)
                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow8),
            )
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow9);
        let mut point_sum: Point = self.points_sum(gauge_type, next_time);
        point_sum.bias = new_sum;
        PointsSum::instance().set(&gauge_type, &next_time, point_sum);
        TimeSum::instance().set(&U256::from(gauge_type), next_time);
        let _total_weight = _total_weight
            .checked_add(
                (new_sum
                    .checked_mul(type_weight)
                    .unwrap_or_revert_with(Error::GaugeControllerMultiply12))
                .checked_sub(
                    old_sum
                        .checked_mul(type_weight)
                        .unwrap_or_revert_with(Error::GaugeControllerMultiply13),
                )
                .unwrap_or_revert_with(Error::GaugeControllerUnderFlow9),
            )
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow10);
        PointsTotal::instance().set(&next_time, _total_weight);
        data::set_time_total(next_time);

        self.emit(&GAUGECONLTROLLEREvent::NewGaugeWeight {
            gauge_address: addr,
            time: U256::from(u64::from(runtime::get_blocktime())),
            weight,
            total_weight: _total_weight,
        });
    }
    #[inline(always)]
    fn gauge_relative_weight(&mut self, addr: Key, time: Option<U256>) -> U256 {
        let time_: U256 = if let Some(..) = time {
            time.unwrap()
        } else {
            U256::from(u64::from(runtime::get_blocktime()))
        };
        self._gauge_relative_weight(addr, time_)
    }
    #[inline(always)]
    fn gauge_relative_weight_write(&mut self, addr: Key, time: Option<U256>) -> U256 {
        let time_: U256 = if let Some(..) = time {
            time.unwrap()
        } else {
            U256::from(u64::from(runtime::get_blocktime()))
        };
        self._get_weight(addr);
        self._get_total(); // Also calculates get_sum
        self._gauge_relative_weight(addr, time_)
    }
    #[inline(always)]
    fn change_type_weight(&mut self, type_id: i128, weight: U256) {
        if self.get_caller() == self.admin() {
            self._change_type_weight(type_id, weight);
        } else {
            runtime::revert(Error::GaugeControllerNotAdmin1);
        }
    }
    #[inline(always)]
    fn change_gauge_weight(&mut self, addr: Key, weight: U256) {
        if self.get_caller() == self.admin() {
            self._change_gauge_weight(addr, weight);
        } else {
            runtime::revert(Error::GaugeControllerNotAdmin2);
        }
    }
    #[inline(always)]
    fn get_gauge_weight(&mut self, addr: Key) -> U256 {
        let time_weight = self.time_weight(addr);
        self.points_weight(addr, time_weight).bias
    }
    #[inline(always)]
    fn get_type_weight(&mut self, type_id: i128) -> U256 {
        let time_type_weight = self.time_type_weight(U256::from(type_id));
        self.points_type_weight(type_id, time_type_weight)
    }
    #[inline(always)]
    fn get_total_weight(&mut self) -> U256 {
        let time_total = self.time_total();
        self.points_total(time_total)
    }
    #[inline(always)]
    fn get_weights_sum_per_type(&mut self, type_id: i128) -> U256 {
        let total_sum = self.time_sum(U256::from(type_id));
        self.points_sum(type_id, total_sum).bias
    }
    #[inline(always)]
    fn changes_sum(&mut self, owner: i128, spender: U256) -> U256 {
        ChangesSum::instance().get(&owner, &spender)
    }
    #[inline(always)]
    fn changs_sum(&mut self, owner: i128, spender: U256) -> U256 {
        ChangesSum::instance().get(&owner, &spender)
    }
    #[inline(always)]
    fn changes_weight(&mut self, owner: Key, spender: U256) -> U256 {
        ChangesWeight::instance().get(&owner, &spender)
    }
    #[inline(always)]
    fn gauge_type_names(&mut self, owner: i128) -> String {
        GaugeTypeNames::instance().get(&owner)
    }
    #[inline(always)]
    fn gauge_types_(&mut self, owner: Key) -> i128 {
        GaugeTypes_::instance().get(&owner)
    }
    #[inline(always)]
    fn gauges(&mut self, owner: U256) -> Key {
        Gauges::instance().get(&owner)
    }
    #[inline(always)]
    fn last_user_vote(&mut self, owner: Key, spender: Key) -> U256 {
        LastUserVote::instance().get(&owner, &spender)
    }
    #[inline(always)]
    fn points_sum(&mut self, owner: i128, spender: U256) -> Point {
        PointsSum::instance().get(&owner, &spender)
    }
    #[inline(always)]
    fn points_total(&mut self, owner: U256) -> U256 {
        PointsTotal::instance().get(&owner)
    }
    #[inline(always)]
    fn points_type_weight(&mut self, owner: i128, spender: U256) -> U256 {
        PointsTypeWeight::instance().get(&owner, &spender)
    }
    #[inline(always)]
    fn points_weight(&mut self, owner: Key, spender: U256) -> Point {
        PointsWeight::instance().get(&owner, &spender)
    }
    #[inline(always)]
    fn time_sum(&mut self, type_id: U256) -> U256 {
        TimeSum::instance().get(&type_id)
    }
    #[inline(always)]
    fn time_type_weight(&mut self, type_id: U256) -> U256 {
        TimeTypeWeight::instance().get(&type_id)
    }
    #[inline(always)]
    fn time_weight(&mut self, owner: Key) -> U256 {
        TimeWeight::instance().get(&owner)
    }
    #[inline(always)]
    fn vote_user_power(&mut self, owner: Key) -> U256 {
        VoteUserPower::instance().get(&owner)
    }
    #[inline(always)]
    fn vote_user_slopes(&mut self, owner: Key, spender: Key) -> VotedSlope {
        VoteUserSlopes::instance().get(&owner, &spender)
    }
    /// TimeWeight, VoteUserPower, VoteUserSlopes,
    #[inline(always)]
    fn add_type(&mut self, _name: String, _weight: Option<U256>) {
        let weight: U256 = if let Some(..) = _weight {
            _weight.unwrap()
        } else {
            0.into()
        };

        if self.get_caller() != data::admin() { 
          runtime::revert(Error::GaugeControllerNotAdmin3);
        }
            
        let type_id: i128 = data::n_gauge_types();
        GaugeTypeNames::instance().set(&type_id, _name.clone());
        data::set_n_gauge_types(
            type_id
                .checked_add(1)
                .unwrap_or_revert_with(Error::GaugeControllerOverFlow11),
        );
        if weight != U256::from(0) {
            self._change_type_weight(type_id, weight);
            self.emit(&GAUGECONLTROLLEREvent::AddType {
                name: _name,
                type_id,
            });
        }
    }

    #[inline(always)]
    fn add_gauge(&mut self, addr: Key, gauge_type: i128, _weight: Option<U256>) {
        let weight: U256 = if let Some(..) = _weight {
            _weight.unwrap()
        } else {
            0.into()
        };

        if self.get_caller() != data::admin() {
          runtime::revert(Error::GaugeControllerNotAdmin4);
        }
            
        if gauge_type < 0 || gauge_type >= data::n_gauge_types() {
          runtime::revert(Error::GaugeControllerCannotAddSameGaugeTwice);
        }
        
        if self.gauge_types_(addr) != 0
        // dev: cannot add the same gauge twice
        {
          runtime::revert(Error::GaugeControllerGaugeType1);
        }

        let n: i128 = data::n_gauges();
        data::set_n_gauges(
            n.checked_add(1)
                .unwrap_or_revert_with(Error::GaugeControllerOverFlow12),
        );
        Gauges::instance().set(&U256::from(n), addr);
        GaugeTypes_::instance().set(
            &addr,
            gauge_type
                .checked_add(1)
                .unwrap_or_revert_with(Error::GaugeControllerOverFlow13),
        );
        let next_time: U256 = (U256::from(u64::from(runtime::get_blocktime()))
            .checked_add(WEEK)
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow14))
        .checked_div(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerDivide6)
        .checked_mul(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerMultiply14);
        if weight > U256::from(0) {
            let mut points_sum_result = self.points_sum(gauge_type, next_time);
            (points_sum_result).bias = weight
                .checked_add(self._get_sum(gauge_type))
                .unwrap_or_revert_with(Error::GaugeControllerOverFlow15);
            PointsSum::instance().set(&gauge_type, &next_time, points_sum_result);
        
            TimeSum::instance().set(&U256::from(gauge_type), next_time);
            PointsTotal::instance().set(
                &next_time,
                self._get_total()
                    .checked_add(
                      self._get_type_weight(gauge_type)
                            .checked_mul(weight)
                            .unwrap_or_revert_with(Error::GaugeControllerMultiply15),
                    )
                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow16),
            );
            data::set_time_total(next_time);
            
            let mut points_weight_result = self.points_weight(addr, next_time);
            (points_weight_result).bias = weight;
            PointsWeight::instance().set(&addr, &next_time, points_weight_result);
        }

        if self.time_sum(U256::from(gauge_type)) == U256::from(0) {
            TimeSum::instance().set(&U256::from(gauge_type), next_time);
        }

        TimeWeight::instance().set(&addr, next_time);

        self.emit(&GAUGECONLTROLLEREvent::NewGauge {
            addr,
            gauge_type,
            weight,
        });
    }

    #[inline(always)]
    fn vote_for_gauge_weights(&mut self, _gauge_addr: Key, _user_weight: U256) {
        let escrow: Key = data::voting_escrow();

        //convert Key to ContractPackageHash
        let escrow_package_hash_add_array = match escrow {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let escrow_package_hash = ContractPackageHash::new(escrow_package_hash_add_array);

        let _slope: (bool, U128) = runtime::call_versioned_contract(
            escrow_package_hash,
            None,
            "get_last_user_slope",
            runtime_args! {"addr" => self.get_caller()},
        );
        let slope = U256::from(tuple_to_i128(_slope));

        let lock_end: U256 = runtime::call_versioned_contract(
            escrow_package_hash,
            None,
            "locked_end",
            runtime_args! {"addr" => self.get_caller()},
        );

        let _n_gauges: i128 = data::n_gauges();
        let next_time: U256 = (U256::from(u64::from(runtime::get_blocktime()))
            .checked_add(WEEK)
            .unwrap_or_revert_with(Error::GaugeControllerOverFlow17))
        .checked_div(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerDivide7)
        .checked_mul(WEEK)
        .unwrap_or_revert_with(Error::GaugeControllerMultiply16);

        if lock_end > next_time {
            if _user_weight >= U256::from(0) && _user_weight <= U256::from(10000) {
                if (U256::from(u64::from(runtime::get_blocktime())))
                    >= (self
                        .last_user_vote(self.get_caller(), _gauge_addr)
                        .checked_add(WEIGHT_VOTE_DELAY)
                        .unwrap_or_revert_with(Error::GaugeControllerOverFlow18))
                {
                    let gauge_type: i128 = self
                        .gauge_types_(_gauge_addr)
                        .checked_sub(1)
                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow10);
                    if gauge_type >= 0 {
                        // Prepare slopes and biases in memory
                        let old_slope: VotedSlope =
                            self.vote_user_slopes(self.get_caller(), _gauge_addr);
                        let mut old_dt: U256 = 0.into();
                        if old_slope.end > next_time {
                            old_dt = old_slope
                                .end
                                .checked_sub(next_time)
                                .unwrap_or_revert_with(Error::GaugeControllerUnderFlow11);
                        }
                        let old_bias: U256 = old_slope
                            .slope
                            .checked_mul(old_dt)
                            .unwrap_or_revert_with(Error::GaugeControllerMultiply17);
                        let new_slope: VotedSlope = VotedSlope {
                            slope: slope
                                .checked_mul(
                                    _user_weight
                                        .checked_div(U256::from(10000))
                                        .unwrap_or_revert_with(Error::GaugeControllerDivide8),
                                )
                                .unwrap_or_revert_with(Error::GaugeControllerMultiply18),
                            end: lock_end,
                            power: _user_weight,
                        };
                        let new_dt: U256 = lock_end
                            .checked_sub(next_time)
                            .unwrap_or_revert_with(Error::GaugeControllerUnderFlow12); // dev: raises when expired
                        let new_bias: U256 = new_slope
                            .slope
                            .checked_mul(new_dt)
                            .unwrap_or_revert_with(Error::GaugeControllerMultiply19);

                        // Check and update powers (weights) used
                        let mut power_used: U256 = self.vote_user_power(self.get_caller());
                        power_used = power_used
                            .checked_add(
                                new_slope
                                    .power
                                    .checked_sub(old_slope.power)
                                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow13),
                            )
                            .unwrap_or_revert_with(Error::GaugeControllerOverFlow19);
                        VoteUserPower::instance().set(&self.get_caller(), power_used);

                        if (power_used >= 0.into()) && (power_used <= 10000.into()) {
                            // Remove old and schedule new slope changes
                            // Remove slope changes for old slopes
                            // Schedule recording of initial slope for next_time

                            let old_weight_bias: U256 = self._get_weight(_gauge_addr);
                            let old_weight_slope: U256 =
                                self.points_weight(_gauge_addr, next_time).slope;
                            let old_sum_bias: U256 = self._get_sum(gauge_type);
                            let old_sum_slope: U256 = self.points_sum(gauge_type, next_time).slope;

                            let max_weight_bias = old_weight_bias
                                .checked_add(new_bias)
                                .unwrap_or_revert_with(Error::GaugeControllerOverFlow20);
                            let max_sum_bias = old_sum_bias
                                .checked_add(new_bias)
                                .unwrap_or_revert_with(Error::GaugeControllerOverFlow21);

                            if max_weight_bias > old_bias {
                                let mut points_weight_result =
                                    self.points_weight(_gauge_addr, next_time);
                                (points_weight_result).bias = max_weight_bias
                                    .checked_sub(old_bias)
                                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow14);
                                PointsWeight::instance().set(
                                    &_gauge_addr,
                                    &next_time,
                                    points_weight_result,
                                );
                            } else {
                                let mut points_weight_result =
                                    self.points_weight(_gauge_addr, next_time);
                                (points_weight_result).bias = old_bias
                                    .checked_sub(old_bias)
                                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow15);
                                PointsWeight::instance().set(
                                    &_gauge_addr,
                                    &next_time,
                                    points_weight_result,
                                );
                            }

                            if max_sum_bias > old_bias {
                                let mut points_sum_result = self.points_sum(gauge_type, next_time);
                                (points_sum_result).bias = max_sum_bias
                                    .checked_sub(old_bias)
                                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow16);
                                PointsSum::instance().set(
                                    &gauge_type,
                                    &next_time,
                                    points_sum_result,
                                );
                            } else {
                                let mut points_sum_result = self.points_sum(gauge_type, next_time);
                                (points_sum_result).bias = old_bias
                                    .checked_sub(old_bias)
                                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow17);
                                PointsSum::instance().set(
                                    &gauge_type,
                                    &next_time,
                                    points_sum_result,
                                );
                            }

                            if old_slope.end > next_time {
                                let max_weight_slope = old_weight_slope
                                    .checked_add(new_slope.slope)
                                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow22);
                                let max_sum_slope = old_sum_slope
                                    .checked_add(new_slope.slope)
                                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow23);

                                if max_weight_slope > old_slope.slope {
                                    let mut points_weight_result =
                                        self.points_weight(_gauge_addr, next_time);
                                    (points_weight_result).slope = max_weight_slope
                                        .checked_sub(old_slope.slope)
                                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow18);
                                    PointsWeight::instance().set(
                                        &_gauge_addr,
                                        &next_time,
                                        points_weight_result,
                                    );
                                } else {
                                    let mut points_weight_result =
                                        self.points_weight(_gauge_addr, next_time);
                                    (points_weight_result).slope = old_slope
                                        .slope
                                        .checked_sub(old_slope.slope)
                                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow19);
                                    PointsWeight::instance().set(
                                        &_gauge_addr,
                                        &next_time,
                                        points_weight_result,
                                    );
                                }

                                if max_sum_slope > old_slope.slope {
                                    let mut points_sum_result =
                                        self.points_sum(gauge_type, next_time);
                                    (points_sum_result).slope = max_sum_slope
                                        .checked_sub(old_slope.slope)
                                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow20);
                                    PointsSum::instance().set(
                                        &gauge_type,
                                        &next_time,
                                        points_sum_result,
                                    );
                                } else {
                                    let mut points_sum_result =
                                        self.points_sum(gauge_type, next_time);
                                    (points_sum_result).slope = old_slope
                                        .slope
                                        .checked_sub(old_slope.slope)
                                        .unwrap_or_revert_with(Error::GaugeControllerUnderFlow21);
                                    PointsSum::instance().set(
                                        &gauge_type,
                                        &next_time,
                                        points_sum_result,
                                    );
                                }
                            } else {
                                let mut points_weight_result =
                                    self.points_weight(_gauge_addr, next_time);
                                (points_weight_result).slope = (points_weight_result)
                                    .slope
                                    .checked_add(new_slope.slope)
                                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow24);
                                PointsWeight::instance().set(
                                    &_gauge_addr,
                                    &next_time,
                                    points_weight_result,
                                );

                                let mut points_sum_result = self.points_sum(gauge_type, next_time);
                                (points_sum_result).slope = (points_sum_result)
                                    .slope
                                    .checked_add(new_slope.slope)
                                    .unwrap_or_revert_with(Error::GaugeControllerOverFlow25);
                                PointsSum::instance().set(
                                    &gauge_type,
                                    &next_time,
                                    points_sum_result,
                                );
                            }

                            if old_slope.end > U256::from(u64::from(runtime::get_blocktime())) {
                                // Cancel old slope changes if they still didn't happen

                                let mut changes_weight_result =
                                    self.changes_weight(_gauge_addr, old_slope.end);
                                changes_weight_result = changes_weight_result
                                    .checked_sub(old_slope.slope)
                                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow22);
                                ChangesWeight::instance().set(
                                    &_gauge_addr,
                                    &old_slope.end,
                                    changes_weight_result,
                                );

                                let mut changes_sum_result =
                                    self.changes_sum(gauge_type, old_slope.end);
                                changes_sum_result = changes_sum_result
                                    .checked_sub(old_slope.slope)
                                    .unwrap_or_revert_with(Error::GaugeControllerUnderFlow23);
                                ChangesSum::instance().set(
                                    &gauge_type,
                                    &old_slope.end,
                                    changes_sum_result,
                                );
                            }

                            // Add slope changes for new slopes

                            let mut changes_weight_result =
                                self.changes_weight(_gauge_addr, new_slope.end);
                            changes_weight_result = changes_weight_result
                                .checked_add(new_slope.slope)
                                .unwrap_or_revert_with(Error::GaugeControllerOverFlow26);
                            ChangesWeight::instance().set(
                                &_gauge_addr,
                                &old_slope.end,
                                changes_weight_result,
                            );

                            let mut changes_sum_result =
                                self.changes_sum(gauge_type, new_slope.end);
                            changes_sum_result = changes_sum_result
                                .checked_add(new_slope.slope)
                                .unwrap_or_revert_with(Error::GaugeControllerOverFlow27);
                            ChangesSum::instance().set(
                                &gauge_type,
                                &old_slope.end,
                                changes_sum_result,
                            );

                            self._get_total();

                            VoteUserSlopes::instance().set(
                                &self.get_caller(),
                                &_gauge_addr,
                                new_slope,
                            );

                            //Record last action time
                            LastUserVote::instance().set(
                                &self.get_caller(),
                                &_gauge_addr,
                                U256::from(u64::from(runtime::get_blocktime())),
                            );

                            self.emit(&GAUGECONLTROLLEREvent::VoteForGauge {
                                time: U256::from(u64::from(runtime::get_blocktime())),
                                user: self.get_caller(),
                                gauge_addr: _gauge_addr,
                                weight: _user_weight,
                            });
                        } else {
                            runtime::revert(Error::GaugeControllerUsedTooMuchPower);
                        }
                    } else {
                        runtime::revert(Error::GaugeControllerGaugeNotAdded);
                    }
                } else {
                    runtime::revert(Error::GaugeControllerCannotVoteSoOften);
                }
            } else {
                runtime::revert(Error::GaugeControllerUsedAllYourVotingPower);
            }
        } else {
            runtime::revert(Error::GaugeControllerTokenLockExpiresTooSoon);
        }
    }

    #[inline(always)]
    fn time_total(&mut self) -> U256 {
        data::time_total()
    }
    #[inline(always)]
    fn token(&mut self) -> Key {
        data::token()
    }
    #[inline(always)]
    fn admin(&mut self) -> Key {
        data::admin()
    }
    #[inline(always)]
    fn future_admin(&mut self) -> Key {
        data::future_admin()
    }
    #[inline(always)]
    fn voting_escrow(&mut self) -> Key {
        data::voting_escrow()
    }
    #[inline(always)]
    fn n_gauge_types(&mut self) -> i128 {
        data::n_gauge_types()
    }
    #[inline(always)]
    fn n_gauges(&mut self) -> i128 {
        data::n_gauges()
    }

    #[inline(always)]
    fn emit(&mut self, gauge_controller_event: &GAUGECONLTROLLEREvent) {
        match gauge_controller_event {
            GAUGECONLTROLLEREvent::Minted {
                recipient,
                gauge,
                minted,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("recipient", recipient.to_string());
                event.insert("gauge", gauge.to_string());
                event.insert("minted", minted.to_string());
                storage::new_uref(event);
            }
            GAUGECONLTROLLEREvent::CommitOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            GAUGECONLTROLLEREvent::ApplyOwnership { admin } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("admin", admin.to_string());
                storage::new_uref(event);
            }
            GAUGECONLTROLLEREvent::NewTypeWeight {
                type_id,
                time,
                weight,
                total_weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("type_id", type_id.to_string());
                event.insert("time", time.to_string());
                event.insert("weight", weight.to_string());
                event.insert("total_weight", total_weight.to_string());
                storage::new_uref(event);
            }
            GAUGECONLTROLLEREvent::NewGaugeWeight {
                gauge_address,
                time,
                weight,
                total_weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("gauge_address", gauge_address.to_string());
                event.insert("time", time.to_string());
                event.insert("weight", weight.to_string());
                event.insert("total_weight", total_weight.to_string());
                storage::new_uref(event);
            }
            GAUGECONLTROLLEREvent::AddType { name, type_id } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("name", name.to_string());
                event.insert("type_id", type_id.to_string());
                storage::new_uref(event);
            }
            GAUGECONLTROLLEREvent::NewGauge {
                addr,
                gauge_type,
                weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("addr", addr.to_string());
                event.insert("gauge_type", gauge_type.to_string());
                event.insert("weight", weight.to_string());
                storage::new_uref(event);
            }
            GAUGECONLTROLLEREvent::VoteForGauge {
                time,
                user,
                gauge_addr,
                weight,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", get_package_hash().to_string());
                event.insert("event_type", gauge_controller_event.type_name());
                event.insert("time", time.to_string());
                event.insert("user", user.to_string());
                event.insert("gauge_addr", gauge_addr.to_string());
                event.insert("weight", weight.to_string());
                storage::new_uref(event);
            }
        };
    }

    #[inline(always)]
    fn get_package_hash(&mut self) -> ContractPackageHash {
        data::get_package_hash()
    }
}
