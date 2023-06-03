use casper_types::ApiError;

#[repr(u16)]
pub enum Error {
    /// ERC20 CRV ERRORS
    /// (ERC20 CRV Invalid Admin1)
    Erc20CRVInvalidAdmin1 = 10001,
    /// (ERC20 CRV Invalid Admin2)
    Erc20CRVInvalidAdmin2 = 10002,
    /// (ERC20 CRV Admin only1)
    Erc20CRVAdminOnly1 = 10003,
    /// (ERC20 CRV Admin Only2)
    Erc20CRVAdminOnly2 = 10004,
    /// (ERC20 CRV Too Soon)
    Erc20CRVTooSoon = 10005,
    /// (ERC20 CRV Zero Address)
    Erc20CRVZeroAddress = 10006,
    /// (ERC20 CRV Minter Only)
    Erc20CRVMinterOnly = 10007,
    /// (ERC20 CRV Exceeds Allowable Mint)
    Erc20CRVExceedsAllowableMint = 10008,
    /// (ERC20 CRV Start Greater Than End)
    Erc20CRVStartGreaterThanEnd = 10009,
    /// (ERC20 CRV Too Far In Future)
    Erc20CRVTooFarInFuture = 10010,
    /// (ERC20 CRV Curr Rate Less Than Init Rate)
    Erc20CRVCurrRateLessThanInitRate = 10011,
    /// (ERC20 CRV Over flow1)
    Erc20CRVOverFlow1 = 10012,
    /// (ERC20 CRV Over flow2)
    Erc20CRVOverFlow2 = 10013,
    /// (ERC20 CRV Airthmetic Error1)
    Erc20CRVAirthmeticError1 = 10014,
    /// (ERC20 CRV Over flow3)
    Erc20CRVOverFlow3 = 10015,
    /// (ERC20 CRV Over flow4)
    Erc20CRVOverFlow4 = 10016,
    /// (ERC20 CRV Over flow5)
    Erc20CRVOverFlow5 = 10017,
    /// (ERC20 CRV Over flow6)
    Erc20CRVOverFlow6 = 10018,
    /// (ERC20 CRV Airthmetic Error2)
    Erc20CRVAirthmeticError2 = 10019,
    /// (ERC20 CRV Over flow7)
    Erc20CRVOverFlow7 = 10020,
    /// (ERC20 CRV Under flow1)
    Erc20CRVUnderFlow1 = 10021,
    /// (ERC20 CRV Airthmetic Error3)
    Erc20CRVAirthmeticError3 = 10022,
    /// (ERC20 CRV Over flow8)
    Erc20CRVOverFlow8 = 10023,
    /// (ERC20 CRV Under flow3)
    Erc20CRVUnderFlow3 = 10024,
    /// (ERC20 CRV Over flow9)
    Erc20CRVOverFlow9 = 10025,
    /// (ERC20 CRV Over flow10)
    Erc20CRVOverFlow10 = 10026,
    /// (ERC20 CRV Airthmetic Error4)
    Erc20CRVAirthmeticError4 = 10027,
    /// (ERC20 CRV Airthmetic Error5)
    Erc20CRVAirthmeticError5 = 10028,
    /// (ERC20 CRV Over flow11)
    Erc20CRVOverFlow11 = 10029,
    /// (ERC20 CRV Over flow12)
    Erc20CRVOverFlow12 = 10030,
    /// (ERC20 CRV Over flow13)
    Erc20CRVOverFlow13 = 10031,
    /// (ERC20 CRV Over flow14)
    Erc20CRVOverFlow14 = 10032,
    /// (ERC20 CRV Over flow15)
    Erc20CRVOverFlow15 = 10033,
    /// (ERC20 CRV Under flow4)
    Erc20CRVUnderFlow4 = 10034,
    /// (ERC20 CRV Over flow16)
    Erc20CRVOverFlow16 = 10035,
    /// (ERC20 CRV Over flow17)
    Erc20CRVOverFlow17 = 10036,
    /// (ERC20 CRV Over flow18)
    Erc20CRVOverFlow18 = 10037,
    /// (Erc20 CRV Already Added)
    Erc20CRVAlreadyAdded = 10038,
    /// (Erc20 CRV Already Removed)
    Erc20CRVAlreadyRemoved = 10039,
    /// (ERC20 CRV Zero Address 1)
    Erc20CRVZeroAddress1 = 10040,
    /// (ERC20 CRV Under flow2)
    Erc20CRVUnderFlow2 = 10041,
    /// (ERC20 CRV Over flow21)
    Erc20CRVOverFlow21 = 10042,
    /// (ERC20 CRV Under flow5)
    Erc20CRVUnderFlow5 = 10043,
    /// (ERC20 CRV Over flow22)
    Erc20CRVOverFlow22 = 10044,
    /// (ERC20 CRV Over flow23)
    Erc20CRVOverFlow23 = 10045,
    /// (ERC20 CRV Over flow24)
    Erc20CRVOverFlow24 = 10046,

    /// (Liquidity Gauge Reward Unauthorized)
    LiquidityGaugeRewardUnauthorized = 10101,
    /// (Liquidity Gauge Reward Kick Not Allowed1)
    LiquidityGaugeRewardKickNotAllowed1 = 10102,
    /// (Liquidity Gauge Reward Kick Not Needed2)
    LiquidityGaugeRewardKickNotNeeded2 = 10103,
    /// (Liquidity Gauge Reward Not Approved)
    LiquidityGaugeRewardNotApproved = 10104,
    /// (Liquidity Gauge Reward Is Locked1)
    LiquidityGaugeRewardIsLocked1 = 10105,
    /// (Liquidity Gauge Reward Is Locked2)
    LiquidityGaugeRewardIsLocked2 = 10106,
    /// (Liquidity Gauge Reward Is Locked3)
    LiquidityGaugeRewardIsLocked3 = 10107,
    /// (Liquidity Gauge Reward Admin Only1)
    LiquidityGaugeRewardAdminOnly1 = 10108,
    /// (Liquidity Gauge Reward Admin Only2)
    LiquidityGaugeRewardAdminOnly2 = 10109,
    /// (Liquidity Gauge Reward Admin Only3)
    LiquidityGaugeRewardAdminOnly3 = 10110,
    /// (Liquidity Gauge Reward Admin Only4)
    LiquidityGaugeRewardAdminOnly4 = 10111,
    /// (Liquidity Gauge Reward Admin Not Set)
    LiquidityGaugeRewardAdminNotSet = 10112,
    /// (Liquidity Gauge Reward Zero Address1)
    LiquidityGaugeRewardZeroAddress1 = 10113,
    /// (Liquidity Gauge Reward Zero Address2)
    LiquidityGaugeRewardZeroAddress2 = 10114,
    /// (Liquidity Gauge Reward Zero Address3)
    LiquidityGaugeRewardZeroAddress3 = 10115,
    /// (Liquidity Gauge Reward Arithmatic Error 1)
    LiquidityGaugeRewardArithmaticError1 = 10116,
    /// (Liquidity Gauge Reward Arithmatic Error 2)
    LiquidityGaugeRewardArithmaticError2 = 10117,
    /// (Liquidity Gauge Reward Arithmatic Error 3)
    LiquidityGaugeRewardArithmaticError3 = 10118,
    /// (Liquidity Gauge Reward Arithmatic Error 4)
    LiquidityGaugeRewardArithmaticError4 = 10119,
    /// (Liquidity Gauge Reward Arithmatic Error 5)
    LiquidityGaugeRewardArithmaticError5 = 10120,
    /// (Liquidity Gauge Reward Arithmatic Error 6)
    LiquidityGaugeRewardArithmaticError6 = 10121,
    /// (Liquidity Gauge Reward Arithmatic Error 7)
    LiquidityGaugeRewardArithmaticError7 = 10122,
    /// (Liquidity Gauge Reward Arithmatic Error 8)
    LiquidityGaugeRewardArithmaticError8 = 10123,
    /// (Liquidity Gauge Reward Arithmatic Error 9)
    LiquidityGaugeRewardArithmaticError9 = 10124,
    /// (Liquidity Gauge Reward Arithmatic Error 10)
    LiquidityGaugeRewardArithmaticError10 = 10125,
    /// (Liquidity Gauge Reward Arithmatic Error 11)
    LiquidityGaugeRewardArithmaticError11 = 10126,
    /// (Liquidity Gauge Reward Arithmatic Error 12)
    LiquidityGaugeRewardArithmaticError12 = 10127,
    /// (Liquidity Gauge Reward Arithmatic Error 13)
    LiquidityGaugeRewardArithmaticError13 = 10128,
    /// (Liquidity Gauge Reward Arithmatic Error 14)
    LiquidityGaugeRewardArithmaticError14 = 10129,
    /// (Liquidity Gauge Reward Arithmatic Error 15)
    LiquidityGaugeRewardArithmaticError15 = 10130,
    /// (Liquidity Gauge Reward Arithmatic Error 16)
    LiquidityGaugeRewardArithmaticError16 = 10131,
    /// (Liquidity Gauge Reward Arithmatic Error 17)
    LiquidityGaugeRewardArithmaticError17 = 10132,
    /// (Liquidity Gauge Reward Arithmatic Error 18)
    LiquidityGaugeRewardArithmaticError18 = 10133,
    /// (Liquidity Gauge Reward Arithmatic Error 19)
    LiquidityGaugeRewardArithmaticError19 = 10134,
    /// (Liquidity Gauge Reward Arithmatic Error 20)
    LiquidityGaugeRewardArithmaticError20 = 10135,
    /// (Liquidity Gauge Reward Arithmatic Error 21)
    LiquidityGaugeRewardArithmaticError21 = 10136,
    /// (Liquidity Gauge Reward Arithmatic Error 22)
    LiquidityGaugeRewardArithmaticError22 = 10137,
    /// (Liquidity Gauge Reward Arithmatic Error 23)
    LiquidityGaugeRewardArithmaticError23 = 10138,
    /// (Liquidity Gauge Reward Arithmatic Error 24)
    LiquidityGaugeRewardArithmaticError24 = 10139,
    /// (Liquidity Gauge Reward Arithmatic Error 25)
    LiquidityGaugeRewardArithmaticError25 = 10140,
    /// (Liquidity Gauge Reward Arithmatic Error 26)
    LiquidityGaugeRewardArithmaticError26 = 10141,
    /// (Liquidity Gauge Reward Arithmatic Error 27)
    LiquidityGaugeRewardArithmaticError27 = 10142,
    /// (Liquidity Gauge Reward Arithmatic Error 28)
    LiquidityGaugeRewardArithmaticError28 = 10143,
    /// (Liquidity Gauge Reward Arithmatic Error 29)
    LiquidityGaugeRewardArithmaticError29 = 10144,
    /// (Liquidity Gauge Reward Arithmatic Error 30)
    LiquidityGaugeRewardArithmaticError30 = 10145,
    /// (Liquidity Gauge Reward Arithmatic Error 31)
    LiquidityGaugeRewardArithmaticError31 = 10146,
    /// (Liquidity Gauge Reward Arithmatic Error 32)
    LiquidityGaugeRewardArithmaticError32 = 10147,
    /// (Liquidity Gauge Reward Arithmatic Error 33)
    LiquidityGaugeRewardArithmaticError33 = 10148,
    /// (Liquidity Gauge Reward Arithmatic Error 34)
    LiquidityGaugeRewardArithmaticError34 = 10149,
    /// (Liquidity Gauge Reward Arithmatic Error 35)
    LiquidityGaugeRewardArithmaticError35 = 10150,
    /// (Liquidity Gauge Reward Arithmatic Error 36)
    LiquidityGaugeRewardArithmaticError36 = 10151,
    /// (Liquidity Gauge Reward Arithmatic Error 37)
    LiquidityGaugeRewardArithmaticError37 = 10152,
    /// (Liquidity Gauge Reward Arithmatic Error 38)
    LiquidityGaugeRewardArithmaticError38 = 10153,
    /// (Liquidity Gauge Reward Arithmatic Error 39)
    LiquidityGaugeRewardArithmaticError39 = 10154,
    /// (Liquidity Gauge Reward Arithmatic Error 40)
    LiquidityGaugeRewardArithmaticError40 = 10155,
    /// (Liquidity Gauge Reward Arithmatic Error 41)
    LiquidityGaugeRewardArithmaticError41 = 10156,
    /// (Liquidity Gauge Reward Arithmatic Error 42)
    LiquidityGaugeRewardArithmaticError42 = 10157,
    /// (Liquidity Gauge Reward Arithmatic Error 43)
    LiquidityGaugeRewardArithmaticError43 = 10158,
    /// (Liquidity Gauge Reward Arithmatic Error 44)
    LiquidityGaugeRewardArithmaticError44 = 10159,
    /// (Liquidity Gauge Reward Arithmatic Error 45)
    LiquidityGaugeRewardArithmaticError45 = 10160,
    /// (Liquidity Gauge Reward Arithmatic Error 46)
    LiquidityGaugeRewardArithmaticError46 = 10161,
    /// (Liquidity Gauge Reward Arithmatic Error 47)
    LiquidityGaugeRewardArithmaticError47 = 10162,
    /// (Liquidity Gauge Reward Arithmatic Error 48)
    LiquidityGaugeRewardArithmaticError48 = 10163,
    /// (Liquidity Gauge Reward Arithmatic Error 49)
    LiquidityGaugeRewardArithmaticError49 = 10164,
    /// (Liquidity Gauge Reward Arithmatic Error 50)
    LiquidityGaugeRewardArithmaticError50 = 10165,
    /// (Liquidity Gauge Reward Arithmatic Error 51)
    LiquidityGaugeRewardArithmaticError51 = 10166,
    /// (Liquidity Gauge Reward Arithmatic Error 52)
    LiquidityGaugeRewardArithmaticError52 = 10167,
    /// (Liquidity Gauge Reward Arithmatic Error 53)
    LiquidityGaugeRewardArithmaticError53 = 10168,
    /// (Liquidity Gauge Reward Arithmatic Error 54)
    LiquidityGaugeRewardArithmaticError54 = 10169,
    /// (Liquidity Gauge Reward Arithmatic Error 55)
    LiquidityGaugeRewardArithmaticError55 = 10170,
    /// (Liquidity Gauge Reward Arithmatic Error 56)
    LiquidityGaugeRewardArithmaticError56 = 10171,
    /// (Liquidity Gauge Reward Arithmatic Error 57)
    LiquidityGaugeRewardArithmaticError57 = 10172,
    /// (Liquidity Gauge Reward Arithmatic Error 58)
    LiquidityGaugeRewardArithmaticError58 = 10173,
    /// (Liquidity Gauge Reward Arithmatic Error 59)
    LiquidityGaugeRewardArithmaticError59 = 10174,
    /// (Liquidity Gauge Reward Arithmatic Error 60)
    LiquidityGaugeRewardArithmaticError60 = 10175,
    /// (Liquidity Gauge Reward Arithmatic Error 61)
    LiquidityGaugeRewardArithmaticError61 = 10176,

    /// (Reward Wrapper Unauthorized)
    RewardWrapperUnauthorized = 10201,
    /// (Reward Wrapper Not Approved)
    RewardWrapperNotApproved = 10202,
    /// (Reward Wrapper IsLocked1)
    RewardWrapperIsLocked1 = 10203,
    /// (Reward Wrapper IsLocked2)
    RewardWrapperIsLocked2 = 10204,
    /// (Reward Wrapper Is Locked3)
    RewardWrapperIsLocked3 = 10205,
    /// (Reward Wrapper Admin Only1)
    RewardWrapperAdminOnly1 = 10206,
    /// (Reward Wrapper Admin Only2)
    RewardWrapperAdminOnly2 = 10207,
    /// (Reward Wrapper Admin Only3)
    RewardWrapperAdminOnly3 = 10208,
    /// (Reward Wrapper Admin Not Set)
    RewardWrapperAdminNotSet = 10209,
    /// (Reward Wrapper IsKilled1)
    RewardWrapperIsKilled1 = 10210,
    /// (Reward Wrapper Is Killed2)
    RewardWrapperIsKilled2 = 10211,
    /// (Reward Wrapper Division Error 1)
    RewardWrapperDivisionError1 = 10212,
    /// (Reward Wrapper Division Error 2)
    RewardWrapperDivisionError2 = 10213,
    /// (Reward Wrapper Division Error 3)
    RewardWrapperDivisionError3 = 10214,
    /// (Reward Wrapper Division Error 4)
    RewardWrapperDivisionError4 = 10215,
    /// (Reward Wrapper Division Error 5)
    RewardWrapperDivisionError5 = 10216,
    /// (Reward Wrapper Division Error 6)
    RewardWrapperDivisionError6 = 10217,
    /// (Reward Wrapper Division Error 7)
    RewardWrapperDivisionError7 = 10218,
    /// (Reward Wrapper Division Error 8)
    RewardWrapperDivisionError8 = 10219,
    /// (Reward Wrapper Addition Error 1)
    RewardWrapperAdditionError1 = 10220,
    /// (Reward Wrapper Addition Error 2)
    RewardWrapperAdditionError2 = 10221,
    /// (Reward Wrapper Addition Error 3)
    RewardWrapperAdditionError3 = 10222,
    /// (Reward Wrapper Addition Error 4)
    RewardWrapperAdditionError4 = 10223,
    /// (Reward Wrapper Addition Error 5)
    RewardWrapperAdditionError5 = 10224,
    /// (Reward Wrapper Addition Error 6)
    RewardWrapperAdditionError6 = 10225,
    /// (Reward Wrapper Addition Error 7)
    RewardWrapperAdditionError7 = 10226,
    /// (Reward Wrapper Addition Error 8)
    RewardWrapperAdditionError8 = 10227,
    /// (Reward Wrapper Addition Error 9)
    RewardWrapperAdditionError9 = 10228,
    /// (Reward Wrapper Addition Error 10)
    RewardWrapperAdditionError10 = 10229,
    /// (Reward Wrapper Subtraction Error 1)
    RewardWrapperSubtractionError1 = 10230,
    /// (Reward Wrapper Subtraction Error 2)
    RewardWrapperSubtractionError2 = 10231,
    /// (Reward Wrapper Subtraction Error 3)
    RewardWrapperSubtractionError3 = 10232,
    /// (Reward Wrapper Subtraction Error 4)
    RewardWrapperSubtractionError4 = 10233,
    /// (Reward Wrapper Subtraction Error 5)
    RewardWrapperSubtractionError5 = 10234,
    /// (Reward Wrapper Subtraction Error 6)
    RewardWrapperSubtractionError6 = 10235,
    /// (Reward Wrapper Subtraction Error 7)
    RewardWrapperSubtractionError7 = 10236,
    /// (Reward Wrapper Subtraction Error 8)
    RewardWrapperSubtractionError8 = 10237,
    /// (Reward Wrapper Subtraction Error 9)
    RewardWrapperSubtractionError9 = 10238,
    /// (Reward Wrapper Subtraction Error 10)
    RewardWrapperSubtractionError10 = 10239,
    /// (Reward Wrapper Subtraction Error 11)
    RewardWrapperSubtractionError11 = 10240,
    /// (Reward Wrapper Subtraction Error 12)
    RewardWrapperSubtractionError12 = 10241,
    /// (Reward Wrapper Multiply Error 1)
    RewardWrapperMultiplyError1 = 10242,
    /// (Reward Wrapper Multiply Error 2)
    RewardWrapperMultiplyError2 = 10243,
    /// (Reward Wrapper Multiply Error 3)
    RewardWrapperMultiplyError3 = 10244,
    /// (Reward Wrapper Multiply Error 4)
    RewardWrapperMultiplyError4 = 10245,
    /// (Reward Wrapper Multiply Error 5)
    RewardWrapperMultiplyError5 = 10246,
    /// (Reward Wrapper Multiply Error 6)
    RewardWrapperMultiplyError6 = 10247,
    /// (Reward Wrapper Multiply Error 7)
    RewardWrapperMultiplyError7 = 10248,
    /// (Reward Wrapper Multiply Error 8)
    RewardWrapperMultiplyError8 = 10249,

    ///Curve token v3 errors
    /// (Curve Token V3 Only Minter Can Set)
    CurveTokenV3OnlyMinterCanSet = 10501,
    /// (Curve Token V3 Only Minter Allowed )
    CurveTokenV3OnlyMinterAllowed = 10502,
    /// (Curve Token V3  Only Minter Allowed2)
    CurveTokenV3OnlyMinterAllowed2 = 10503,
    /// (Curve Token V3 Not Authorized)
    CurveTokenV3NotAuthorized = 10504,

    // FeeDistributor
    FeeDistributorInvalidTokenCheckpointUpdate = 10601,
    /// (Fee Distributor Killed1)
    FeeDistributorKilled1 = 10602,
    /// (Fee Distributor Killed2)
    FeeDistributorKilled2 = 10603,
    /// (Fee Distributor Killed3)
    FeeDistributorKilled3 = 10604,
    /// (Fee Distributor Is Locked1)
    FeeDistributorIsLocked1 = 10605,
    /// (Fee Distributor Is Locked2)
    FeeDistributorIsLocked2 = 10606,
    /// (Fee Distributor Invalid Coin1)
    FeeDistributorInvalidCoin1 = 10607,
    /// (Fee Distributor Invalid Coin2)
    FeeDistributorInvalidCoin2 = 10608,
    /// (Fee Distributor Access Denied)
    FeeDistributorAccessDenied = 10609,
    /// (Fee Distributor Zero Future Admin)
    FeeDistributorZeroFutureAdmin = 10610,
    /// (Fee Distributor Invalid Admin1)
    FeeDistributorInvalidAdmin1 = 10611,
    /// (Fee Distributor Invalid Admin2)
    FeeDistributorInvalidAdmin2 = 10612,
    /// (Fee Distributor Invalid Admin3)
    FeeDistributorInvalidAdmin3 = 10613,
    /// (Fee Distributor Invalid Admin4)
    FeeDistributorInvalidAdmin4 = 10614,
    /// (Fee Distributor Division1)
    FeeDistributorDivisionError1 = 10615,
    /// (Fee Distributor Division2)
    FeeDistributorDivisionError2 = 10616,
    /// (Fee Distributor Division3)
    FeeDistributorDivisionError3 = 10617,
    /// (Fee Distributor Division4)
    FeeDistributorDivisionError4 = 10618,
    /// (Fee Distributor Division5)
    FeeDistributorDivisionError5 = 10619,
    /// (Fee Distributor Division6)
    FeeDistributorDivisionError6 = 10620,
    ///(Fee Distributor Division7)
    FeeDistributorDivisionError7 = 10621,
    /// (Fee Distributor Division8)
    FeeDistributorDivisionError8 = 10622,
    /// (Fee Distributor Division9)
    FeeDistributorDivisionError9 = 10623,
    /// (Fee Distributor Division10)
    FeeDistributorDivisionError10 = 10624,
    /// (Fee Distributor Division11)
    FeeDistributorDivisionError11 = 10625,
    ///(Fee Distributor Subtraction1)
    FeeDistributorSubtractionError1 = 10626,
    ///(Fee Distributor Subtraction2)
    FeeDistributorSubtractionError2 = 10627,
    ///(Fee Distributor Subtraction3)
    FeeDistributorSubtractionError3 = 10628,
    /// (Fee Distributor Division5)
    FeeDistributorDivisionError12 = 10629,
    ///(Fee Distributor Subtraction5)
    FeeDistributorSubtractionError5 = 10630,
    ///(Fee Distributor Subtraction6)
    FeeDistributorSubtractionError6 = 10631,
    ///(Fee Distributor Subtraction7)
    FeeDistributorSubtractionError7 = 10632,
    ///(Fee Distributor Subtraction8)
    FeeDistributorSubtractionError8 = 10633,
    ///(Fee Distributor Subtraction9)
    FeeDistributorSubtractionError9 = 10634,
    ///(Fee Distributor Subtraction11)
    FeeDistributorSubtractionError11 = 10635,
    ///(Fee Distributor Subtraction12)
    FeeDistributorSubtractionError12 = 10636,
    /// (Fee Distributor Subtraction13)
    FeeDistributorSubtractionError13 = 10637,
    /// (Fee Distributor Subtraction14)
    FeeDistributorSubtractionError14 = 10638,
    /// (Fee Distributor Subtraction15)
    FeeDistributorSubtractionError15 = 10639,
    /// (Fee Distributor Subtraction16)
    FeeDistributorSubtractionError16 = 10640,
    /// (Fee Distributor Subtraction17)
    FeeDistributorSubtractionError17 = 10641,
    /// (Fee Distributor Addition1)
    FeeDistributorAdditionError1 = 10642,
    /// (Fee Distributor Addition2)
    FeeDistributorAdditionError2 = 10643,
    /// (Fee Distributor Addition3)
    FeeDistributorAdditionError3 = 10644,
    /// (Fee Distributor Addition4)
    FeeDistributorAdditionError4 = 10645,
    /// (Fee Distributor Addition5)
    FeeDistributorAdditionError5 = 10646,
    /// (Fee Distributor Addition6)
    FeeDistributorAdditionError6 = 10647,
    /// (Fee Distributor Addition7)
    FeeDistributorAdditionError7 = 10648,
    /// (Fee Distributor Addition8)
    FeeDistributorAdditionError8 = 10649,
    /// (Fee Distributor Addition9)
    FeeDistributorAdditionError9 = 10650,
    /// (Fee Distributor Addition10)
    FeeDistributorAdditionError10 = 10651,
    /// (Fee Distributor Addition11)
    FeeDistributorAdditionError11 = 10652,
    /// (Fee Distributor Addition12)
    FeeDistributorAdditionError12 = 10653,
    /// (Fee Distributor Addition13)
    FeeDistributorAdditionError13 = 10654,
    /// (Fee Distributor Addition14)
    FeeDistributorAdditionError14 = 10655,
    /// (Fee Distributor Addition15)
    FeeDistributorAdditionError15 = 10656,
    /// (Fee Distributor Addition16)
    FeeDistributorAdditionError16 = 10657,
    /// (Fee Distributor Addition17)
    FeeDistributorAdditionError17 = 10658,
    /// (Fee Distributor Addition18)
    FeeDistributorAdditionError18 = 10659,
    /// (Fee Distributor Addition19)
    FeeDistributorAdditionError19 = 10660,
    /// (Fee Distributor Multiplication1)
    FeeDistributorMultiplicationError1 = 10661,
    /// (Fee Distributor Multiplication2)
    FeeDistributorMultiplicationError2 = 10662,
    /// (Fee Distributor Multiplication3)
    FeeDistributorMultiplicationError3 = 10663,
    /// (Fee Distributor Multiplication4)
    FeeDistributorMultiplicationError4 = 10664,
    /// (Fee Distributor Multiplication5)
    FeeDistributorMultiplicationError5 = 10665,
    /// (Fee Distributor Multiplication6)
    FeeDistributorMultiplicationError6 = 10666,
    /// (Fee Distributor Multiplication7)
    FeeDistributorMultiplicationError7 = 10667,
    /// (Fee Distributor Multiplication8)
    FeeDistributorMultiplicationError8 = 10668,
    /// (Fee Distributor Multiplication9)
    FeeDistributorMultiplicationError9 = 10669,
    /// (Fee Distributor Multiplication10)
    FeeDistributorMultiplicationError10 = 10670,
    /// (Fee Distributor Multiplication11)
    FeeDistributorMultiplicationError11 = 10671,
    /// (Fee Distributor Multiplication12)
    FeeDistributorMultiplicationError12 = 10672,
    /// (Fee Distributor Addition 20)
    FeeDistributorAdditionError20 = 10673,
    /// (Fee Distributor Addition 21)
    FeeDistributorAdditionError21 = 10674,

    // Gauge Controller
    /// (Gauge Controller Address Zero1)
    GaugeControllerAddressZero1 = 10701,
    /// (Gauge Controller Address Zero2)
    GaugeControllerAddressZero2 = 10702,
    /// (Gauge Controller Only Admin1)
    GaugeControllerOnlyAdmin1 = 10703,
    /// (Gauge Controller Only Admin2)
    GaugeControllerOnlyAdmin2 = 10704,
    /// (Gauge Controller Admin Not Set)
    GaugeControllerAdminNotSet = 10705,
    /// (Gauge Controller Gauge Type Is Zero)
    GaugeControllerGaugeTypeIsZero = 10706,
    /// (Gauge Controller Not Admin1)
    GaugeControllerNotAdmin1 = 10707,
    /// (Gauge Controller Not Admin2)
    GaugeControllerNotAdmin2 = 10708,
    ///(Gauge Controller Not Admin3)
    GaugeControllerNotAdmin3 = 10709,
    /// (Gauge Controller Not Admin3)
    GaugeControllerNotAdmin4 = 10710,
    /// (Gauge Controller cannot add same gauge twice)
    GaugeControllerCannotAddSameGaugeTwice = 10711,
    /// (Gauge Controller gauge type is greater than equal to zero and less than n_gauge_types)
    GaugeControllerGaugeType1 = 10712,
    /// (Gauge Controller Your token lock expires too soon)
    GaugeControllerTokenLockExpiresTooSoon = 10713,
    /// (Gauge Controller You used all your voting power)
    GaugeControllerUsedAllYourVotingPower = 10714,
    /// (Gauge Controller You Cannot vote so often)
    GaugeControllerCannotVoteSoOften = 10715,
    /// (Gauge Controller Gauge not added)
    GaugeControllerGaugeNotAdded = 10716,
    /// (Gauge Controller Used too much power)
    GaugeControllerUsedTooMuchPower = 10717,
    /// (Gauge Controller OverFlow1)
    GaugeControllerOverFlow1 = 10718,
    /// (Gauge Controller OverFlow2)
    GaugeControllerOverFlow2 = 10719,
    /// (Gauge Controller OverFlow3)
    GaugeControllerOverFlow3 = 10720,
    /// (Gauge Controller OverFlow4)
    GaugeControllerOverFlow4 = 10721,
    /// (Gauge Controller OverFlow5)
    GaugeControllerOverFlow5 = 10722,
    /// (Gauge Controller OverFlow6)
    GaugeControllerOverFlow6 = 10723,
    /// (Gauge Controller OverFlow7)
    GaugeControllerOverFlow7 = 10724,
    /// (Gauge Controller OverFlow8)
    GaugeControllerOverFlow8 = 10725,
    /// (Gauge Controller OverFlow9)
    GaugeControllerOverFlow9 = 10726,
    /// (Gauge Controller OverFlow10)
    GaugeControllerOverFlow10 = 10727,
    /// (Gauge Controller OverFlow11)
    GaugeControllerOverFlow11 = 10728,
    /// (Gauge Controller OverFlow12)
    GaugeControllerOverFlow12 = 10729,
    /// (Gauge Controller OverFlow13)
    GaugeControllerOverFlow13 = 10730,
    /// (Gauge Controller OverFlow14)
    GaugeControllerOverFlow14 = 10731,
    /// (Gauge Controller OverFlow15)
    GaugeControllerOverFlow15 = 10732,
    /// (Gauge Controller OverFlow16)
    GaugeControllerOverFlow16 = 10733,
    /// (Gauge Controller OverFlow17)
    GaugeControllerOverFlow17 = 10734,
    /// (Gauge Controller OverFlow18)
    GaugeControllerOverFlow18 = 10735,
    /// (Gauge Controller OverFlow19)
    GaugeControllerOverFlow19 = 10736,
    /// (Gauge Controller OverFlow20)
    GaugeControllerOverFlow20 = 10737,
    /// (Gauge Controller OverFlow21)
    GaugeControllerOverFlow21 = 10738,
    /// (Gauge Controller OverFlow22)
    GaugeControllerOverFlow22 = 10739,
    /// (Gauge Controller OverFlow23)
    GaugeControllerOverFlow23 = 10740,
    /// (Gauge Controller OverFlow24)
    GaugeControllerOverFlow24 = 10741,
    /// (Gauge Controller OverFlow25)
    GaugeControllerOverFlow25 = 10742,
    /// (Gauge Controller OverFlow26)
    GaugeControllerOverFlow26 = 10743,
    /// (Gauge Controller OverFlow27)
    GaugeControllerOverFlow27 = 10744,
    /// (Gauge Controller UnderFlow1)
    GaugeControllerUnderFlow1 = 10745,
    /// (Gauge Controller UnderFlow2)
    GaugeControllerUnderFlow2 = 10746,
    /// (Gauge Controller UnderFlow3)
    GaugeControllerUnderFlow3 = 10747,
    /// (Gauge Controller UnderFlow4)
    GaugeControllerUnderFlow4 = 10748,
    /// (Gauge Controller UnderFlow5)
    GaugeControllerUnderFlow5 = 10749,
    /// (Gauge Controller UnderFlow6)
    GaugeControllerUnderFlow6 = 10750,
    /// (Gauge Controller UnderFlow7)
    GaugeControllerUnderFlow7 = 10751,
    /// (Gauge Controller UnderFlow8)
    GaugeControllerUnderFlow8 = 10752,
    /// (Gauge Controller UnderFlow9)
    GaugeControllerUnderFlow9 = 10753,
    /// (Gauge Controller UnderFlow10)
    GaugeControllerUnderFlow10 = 10754,
    /// (Gauge Controller UnderFlow11)
    GaugeControllerUnderFlow11 = 10755,
    /// (Gauge Controller UnderFlow12)
    GaugeControllerUnderFlow12 = 10756,
    /// (Gauge Controller UnderFlow13)
    GaugeControllerUnderFlow13 = 10757,
    /// (Gauge Controller UnderFlow14)
    GaugeControllerUnderFlow14 = 10758,
    /// (Gauge Controller UnderFlow15)
    GaugeControllerUnderFlow15 = 10759,
    /// (Gauge Controller UnderFlow16)
    GaugeControllerUnderFlow16 = 10760,
    /// (Gauge Controller UnderFlow17)
    GaugeControllerUnderFlow17 = 10761,
    /// (Gauge Controller UnderFlow18)
    GaugeControllerUnderFlow18 = 10762,
    /// (Gauge Controller UnderFlow19)
    GaugeControllerUnderFlow19 = 10763,
    /// (Gauge Controller UnderFlow20)
    GaugeControllerUnderFlow20 = 10764,
    /// (Gauge Controller UnderFlow21)
    GaugeControllerUnderFlow21 = 10765,
    /// (Gauge Controller UnderFlow22)
    GaugeControllerUnderFlow22 = 10766,
    /// (Gauge Controller UnderFlow23)
    GaugeControllerUnderFlow23 = 10767,
    /// (Gauge Controller UnderFlow23)
    GaugeControllerUnderFlow24 = 10768,
    /// (Gauge Controller Multiply1)
    GaugeControllerMultiply1 = 10769,
    /// (Gauge Controller Multiply2)
    GaugeControllerMultiply2 = 10770,
    ///(Gauge Controller Multiply3)
    GaugeControllerMultiply3 = 10771,
    ///(Gauge Controller Multiply4)
    GaugeControllerMultiply4 = 10772,
    ///(Gauge Controller Multiply5)
    GaugeControllerMultiply5 = 10773,
    ///(Gauge Controller Multiply6)
    GaugeControllerMultiply6 = 10774,
    ///(Gauge Controller Multiply7)
    GaugeControllerMultiply7 = 10775,
    ///(Gauge Controller Multiply8)
    GaugeControllerMultiply8 = 10776,
    ///(Gauge Controller Multiply9)
    GaugeControllerMultiply9 = 10777,
    ///(Gauge Controller Multiply10)
    GaugeControllerMultiply10 = 10778,
    ///(Gauge Controller Multiply11)
    GaugeControllerMultiply11 = 10779,
    ///(Gauge Controller Multiply12)
    GaugeControllerMultiply12 = 10780,
    ///(Gauge Controller Multiply13)
    GaugeControllerMultiply13 = 10781,
    ///(Gauge Controller Multiply14)
    GaugeControllerMultiply14 = 10782,
    ///(Gauge Controller Multiply15)
    GaugeControllerMultiply15 = 10783,
    ///(Gauge Controller Multiply16)
    GaugeControllerMultiply16 = 10784,
    ///(Gauge Controller Multiply17)
    GaugeControllerMultiply17 = 10785,
    ///(Gauge Controller Multiply18)
    GaugeControllerMultiply18 = 10786,
    ///(Gauge Controller Multiply19)
    GaugeControllerMultiply19 = 10787,
    ///(Gauge Controller Divide1)
    GaugeControllerDivide1 = 10788,
    ///(Gauge Controller Divide2)
    GaugeControllerDivide2 = 10789,
    ///(Gauge Controller Divide3)
    GaugeControllerDivide3 = 10790,
    ///(Gauge Controller Divide4)
    GaugeControllerDivide4 = 10791,
    ///(Gauge Controller Divide5)
    GaugeControllerDivide5 = 10792,
    ///(Gauge Controller Divide6)
    GaugeControllerDivide6 = 10793,
    ///(Gauge Controller Divide7)
    GaugeControllerDivide7 = 10794,
    ///(Gauge Controller Divide8)
    GaugeControllerDivide8 = 10795,
    ///(Gauge Controller OverFlow28)
    GaugeControllerOverFlow28 = 10796,
    ///(Gauge Controller OverFlow29)
    GaugeControllerOverFlow29 = 10797,
    ///(Gauge Controller OverFlow30)
    GaugeControllerOverFlow30 = 10798,

    ///(Minter Gauge Is Not Added)
    MinterIsNotAdded = 10801,
    ///(Minter Gauge Locked)
    MinterLocked1 = 10802,
    ///(Minter Gauge Locked)
    MinterLocked2 = 10803,
    ///(Minter Gauge Locked)
    MinterLocked3 = 10804,

    /// (Reward Only Gauge OverFlow1)
    RewardOnlyGaugeOverFlow1 = 10901,
    /// (Reward Only Gauge OverFlow2)
    RewardOnlyGaugeOverFlow2 = 10902,
    /// (Reward Only Gauge OverFlow3)
    RewardOnlyGaugeOverFlow3 = 10903,
    /// (Reward Only Gauge OverFlow4)
    RewardOnlyGaugeOverFlow4 = 10904,
    /// (Reward Only Gauge UnderFlow1)
    RewardOnlyGaugeUnderFlow1 = 10905,
    /// (Reward Only Gauge UnderFlow2)
    RewardOnlyGaugeUnderFlow2 = 10906,
    /// (Reward Only Gauge UnderFlow3)
    RewardOnlyGaugeUnderFlow3 = 10907,
    /// (Reward Only Gauge UnderFlow4)
    RewardOnlyGaugeUnderFlow4 = 10908,
    /// (Reward Only Gauge UnderFlow5)
    RewardOnlyGaugeUnderFlow5 = 10909,
    /// (Reward Only Gauge UnderFlow6)
    RewardOnlyGaugeUnderFlow6 = 10910,
    /// (Reward Only Gauge Only Admin1)
    RewardOnlyGaugeOnlyAdmin1 = 10911,
    /// (Reward Only Gauge Only Admin2)
    RewardOnlyGaugeOnlyAdmin2 = 10912,
    /// (Reward Only Gauge Only Future Admin)
    RewardOnlyGaugeOnlyFutureAdmin = 10913,
    /// (Reward Only Gauge Cannot Redirect When Claiming For Another User)
    RewardOnlyGaugeCannotRedirectWhenClaimingForAnotherUser = 10914,
    /// (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero1 = 10915,
    /// (Reward Only Gauge Value Is Zero)
    RewardOnlyGaugeValueIsZero2 = 10916,
    /// (Reward Only Gauge Reward Token Is Zero)
    RewardOnlyGaugeRewardTokenIsZeroAddress = 10917,
    /// (Reward Only Gauge Cannot Modify Existing Reward Token)
    RewardOnlyGaugeCannotModifyExistingRewardToken = 10918,
    /// (Reward Only Gauge Receiver Is Zero Address)
    RewardOnlyGaugeLocked1 = 10919,
    /// (Reward Only Gauge OverFlow1)
    RewardOnlyGaugeOverFlow5 = 10920,

    /// (Vesting Escrow OverFlow1)
    VestingEscrowOverFlow1 = 11001,
    /// (Vesting Escrow OverFlow2)
    VestingEscrowOverFlow2 = 11002,
    /// (Vesting Escrow OverFlow3)
    VestingEscrowOverFlow3 = 11003,
    /// (Vesting Escrow OverFlow4)
    VestingEscrowOverFlow4 = 11004,
    /// (Vesting Escrow OverFlow5)
    VestingEscrowOverFlow5 = 11005,
    /// (Vesting Escrow UnderFlow1)
    VestingEscrowUnderFlow1 = 11006,
    /// (Vesting Escrow UnderFlow2)
    VestingEscrowUnderFlow2 = 11007,
    /// (Vesting Escrow UnderFlow3)
    VestingEscrowUnderFlow3 = 11008,
    ///(Vesting Escrow UnderFlow4)
    VestingEscrowUnderFlow4 = 11009,
    /// (Vesting Escrow UnderFlow5)
    VestingEscrowUnderFlow5 = 11010,
    /// (Vesting Escrow UnderFlow6)
    VestingEscrowUnderFlow6 = 11011,
    /// (Vesting Escrow UnderFlow7)
    VestingEscrowUnderFlow7 = 11012,
    /// (Vesting Escrow UnderFlow8)
    VestingEscrowUnderFlow8 = 11013,
    /// (Vesting Escrow UnderFlow9)
    VestingEscrowUnderFlow9 = 11014,
    /// (Vesting Escrow UnderFlow10)
    VestingEscrowUnderFlow10 = 11015,
    /// (Vesting Escrow UnderFlow11)
    VestingEscrowUnderFlow11 = 11016,
    /// (Vesting Escrow UnderFlow12)
    VestingEscrowUnderFlow12 = 11017,
    /// (Vesting Escrow UnderFlow13)
    VestingEscrowUnderFlow13 = 11018,
    /// (Vesting Escrow Cannot Disable)
    VestingEscrowCannotDisable = 11019,
    /// (Vesting Escrow Only Admin1)
    VestingEscrowOnlyAdmin1 = 11020,
    /// (Vesting Escrow Only Admin2)
    VestingEscrowOnlyAdmin2 = 11021,
    /// (Vesting Escrow Only Admin3)
    VestingEscrowOnlyAdmin3 = 11022,
    /// (Vesting Escrow Only Admin4)
    VestingEscrowOnlyAdmin4 = 11023,
    /// (Vesting Escrow Only Admin5)
    VestingEscrowOnlyAdmin5 = 11024,
    /// (Vesting Escrow Only Admin6)
    VestingEscrowOnlyAdmin6 = 11025,
    /// (Vesting Escrow Only Admin7)
    VestingEscrowOnlyAdmin7 = 11026,
    /// (Vesting Escrow Admin Not Set)
    VestingEscrowAdminNotSet = 11027,
    /// (Vesting Escrow Locked)
    VestingEscrowLocked1 = 11028,
    /// (Vesting Escrow Locked)
    VestingEscrowLocked2 = 11029,
    /// (Vesting Escrow Fund Admin Disabled)
    VestingEscrowFundAdminsDisabled = 11030,
    /// (Vesting Escrow Factory OverFlow1)
    VestingEscrowFactoryOverFlow1 = 11101,
    /// (Vesting Escrow Factory Only Admin1)
    VestingEscrowFactoryOnlyAdmin1 = 11102,
    /// (Vesting Escrow Factory Only Admin2)
    VestingEscrowFactoryOnlyAdmin2 = 11103,
    /// (Vesting Escrow Factory Only Admin3)
    VestingEscrowFactoryOnlyAdmin3 = 11104,
    /// (Vesting Escrow Factory Duration Too Short)
    VestingEscrowFactoryDurationTooShort = 11105,
    /// (Vesting Escrow Factory Start Time Too Soon)
    VestingEscrowFactoryStartTimeTooSoon = 11106,
    /// (Vesting Escrow Factory Admin Not Set)
    VestingEscrowFactoryAdminNotSet = 11107,

    ///Vesting Escrow simple errors
    /// (Vesting Escrow Simple Initialize Once)
    VestingEscrowSimpleOnlyInitializeOnce = 11201,
    /// (Vesting Escrow Simple Admin Only1)
    VestingEscrowSimpleAdminOnly1 = 11202,
    /// (Vesting Escrow Simple Admin Only2)
    VestingEscrowSimpleAdminOnly2 = 11203,
    /// (Vesting Escrow Simple Admin Only3)
    VestingEscrowSimpleAdminOnly3 = 11204,
    /// (Vesting Escrow Simple Admin Only4)
    VestingEscrowSimpleAdminOnly4 = 11205,
    /// (Vesting Escrow Simple Cannot Disable)
    VestingEscrowSimpleCannotDisable = 11206,
    /// (Vesting Escrow Simple Admin Not Set)
    VestingEscrowSimpleAdminNotSet = 11207,
    /// (Vesting Escrow Simple Is Locked1)
    VestingEscrowSimpleLocked1 = 11208,
    /// (Vesting Escrow Simple Is Locked2)
    VestingEscrowSimpleLocked2 = 11209,
    /// (Vesting Escrow Simple Airthmetic Error1)
    VestingEscrowSimpleAirthmeticError1 = 11210,
    /// (Vesting Escrow Simple Airthmetic Error2)
    VestingEscrowSimpleAirthmeticError2 = 11211,
    /// (Vesting Escrow Simple Under flow1)
    VestingEscrowSimpleUnderFlow1 = 11212,
    /// (Vesting Escrow Simple Under flow2)
    VestingEscrowSimpleUnderFlow2 = 11213,
    /// (Vesting Escrow Simple Under flow3)
    VestingEscrowSimpleUnderFlow3 = 11214,
    /// (Vesting Escrow Simple Under flow4)
    VestingEscrowSimpleUnderFlow4 = 11215,

    /// (Voting Escrow Invalid Decimals)
    VotingEscrowInvalidDecimals = 11301,
    /// (Voting Escrow Admin Only)
    VotingEscrowAdminOnly = 11302,
    /// (Voting Escrow Zero Address)
    VotingEscrowZeroAddress = 11303,
    /// (Voting Escrow Is Locked1)
    VotingEscrowIsLocked1 = 11304,
    /// (Voting Escrow Is Locked2)
    VotingEscrowIsLocked2 = 11305,
    /// (Voting Escrow Is Locked3)
    VotingEscrowIsLocked3 = 11306,
    /// (Voting Escrow Is Locked4)
    VotingEscrowIsLocked4 = 11307,
    /// (Voting Escrow Need Non Zero Value1)
    VotingEscrowNeedNonZeroValue1 = 11308,
    /// (Voting Escrow Need Non Zero Value2)
    VotingEscrowNeedNonZeroValue2 = 11309,
    /// (Voting Escrow Need Non Zero Value3)
    VotingEscrowNeedNonZeroValue3 = 11310,
    /// (Voting Escrow No Existing Lock Found1)
    VotingEscrowNoExistingLockFound1 = 11311,
    /// (Voting Escrow No Existing Lock Found2)
    VotingEscrowNoExistingLockFound2 = 11312,
    /// (Voting Escrow Cannot Add To Expired Lock Withdraw1)
    VotingEscrowCannotAddToExpiredLockWithdraw1 = 11313,
    /// (Voting Escrow Cannot Add To Expired Lock Withdraw2)
    VotingEscrowCannotAddToExpiredLockWithdraw2 = 11314,
    /// (Voting Escrow Withdraw Old Tokens First)
    VotingEscrowWithdrawOldTokensFirst = 11315,
    /// (Voting Escrow Can Only Lock Until Time In The Future)
    VotingEscrowCanOnlyLockUntilTimeInTheFuture = 11316,
    /// (Voting Escrow Voting Lock Can Be 4 Years Max1)
    VotingEscrowVotingLockCanBe4YearsMax1 = 11317,
    /// (Voting Escrow Voting Lock Can Be 4 Years Max2)
    VotingEscrowVotingLockCanBe4YearsMax2 = 11318,
    /// (Voting Escrow Lock Expired)
    VotingEscrowLockExpired = 11319,
    /// (Voting Escrow Is Locked)
    VotingEscrowNothingIsLocked = 11320,
    /// (Voting Escrow Can Only Increase Lock Duration)
    VotingEscrowCanOnlyIncreaseLockDuration = 11321,
    /// (Voting Escrow The Lock Didnt Expire)
    VotingEscrowTheLockDidntExpire = 11322,
    /// (Voting Escrow Division Error 1)
    VotingEscrowDivisionError1 = 11323,
    /// (Voting Escrow Division Error 2)
    VotingEscrowDivisionError2 = 11324,
    /// (Voting Escrow Division Error 3)
    VotingEscrowDivisionError3 = 11325,
    ///(Voting Escrow Division Error 4)
    VotingEscrowDivisionError4 = 11326,
    /// (Voting Escrow Division Error 5)
    VotingEscrowDivisionError5 = 11327,
    /// (Voting Escrow Division Error 6)
    VotingEscrowDivisionError6 = 11328,
    /// (Voting Escrow Division Error 7)
    VotingEscrowDivisionError7 = 11329,
    /// (Voting Escrow Division Error 8)
    VotingEscrowDivisionError8 = 11330,
    ///(Voting Escrow Subtraction Error 1)
    VotingEscrowSubtractionError1 = 11331,
    ///(Voting Escrow Subtraction Error 2)
    VotingEscrowSubtractionError2 = 11332,
    ///(Voting Escrow Subtraction Error 3)
    VotingEscrowSubtractionError3 = 11333,
    ///(Voting Escrow Subtraction Error 4)
    VotingEscrowSubtractionError4 = 11334,
    ///(Voting Escrow Subtraction Error 5)
    VotingEscrowSubtractionError5 = 11335,
    ///(Voting Escrow Subtraction Error 6)
    VotingEscrowSubtractionError6 = 11336,
    ///(Voting Escrow Subtraction Error 7)
    VotingEscrowSubtractionError7 = 11337,
    ///(Voting Escrow Subtraction Error 8)
    VotingEscrowSubtractionError8 = 11338,
    /// (Voting Escrow Subtraction Error 9)
    VotingEscrowSubtractionError9 = 11339,
    /// (Voting Escrow Subtraction Error 10)
    VotingEscrowSubtractionError10 = 11340,
    /// (Voting Escrow Subtraction Error 11)
    VotingEscrowSubtractionError11 = 11341,
    /// (Voting Escrow Subtraction Error 12)
    VotingEscrowSubtractionError12 = 11342,
    /// (Voting Escrow Subtraction Error 13)
    VotingEscrowSubtractionError13 = 11343,
    /// (Voting Escrow Subtraction Error 14)
    VotingEscrowSubtractionError14 = 11344,
    /// (Voting Escrow Subtraction Error 15)
    VotingEscrowSubtractionError15 = 11345,
    /// (Voting Escrow Subtraction Error 16)
    VotingEscrowSubtractionError16 = 11346,
    /// (Voting Escrow Subtraction Error 17)
    VotingEscrowSubtractionError17 = 11347,
    /// (Voting Escrow Subtraction Error 18)
    VotingEscrowSubtractionError18 = 11348,
    /// (Voting Escrow Subtraction Error 19)
    VotingEscrowSubtractionError19 = 11349,
    /// (Voting Escrow Subtraction Error 20)
    VotingEscrowSubtractionError20 = 11350,
    /// (Voting Escrow Subtraction Error 21)
    VotingEscrowSubtractionError21 = 11351,
    /// (Voting Escrow Subtraction Error 22)
    VotingEscrowSubtractionError22 = 11352,
    /// (Voting Escrow Subtraction Error 23)
    VotingEscrowSubtractionError23 = 11353,
    /// (Voting Escrow Subtraction Error 24)
    VotingEscrowSubtractionError24 = 11354,
    /// (Voting Escrow Multiplication Error 1)
    VotingEscrowMultiplicationError1 = 11355,
    /// (Voting Escrow Multiplication Error 2)
    VotingEscrowMultiplicationError2 = 11356,
    /// (Voting Escrow Multiplication Error 3)
    VotingEscrowMultiplicationError3 = 11357,
    /// (Voting Escrow MulVotingEscrowMultiplicationError1tiplication Error 4)
    VotingEscrowMultiplicationError4 = 11358,
    /// (Voting Escrow Multiplication Error 5)
    VotingEscrowMultiplicationError5 = 11359,
    /// (Voting Escrow Multiplication Error 6)
    VotingEscrowMultiplicationError6 = 11360,
    /// (Voting Escrow Multiplication Error 7)
    VotingEscrowMultiplicationError7 = 11361,
    /// (Voting Escrow Multiplication Error 8)
    VotingEscrowMultiplicationError8 = 11362,
    /// (Voting Escrow Multiplication Error 9)
    VotingEscrowMultiplicationError9 = 11363,
    /// (Voting Escrow Multiplication Error 10)
    VotingEscrowMultiplicationError10 = 11364,
    /// (Voting Escrow Multiplication Error 11)
    VotingEscrowMultiplicationError11 = 11365,
    /// (Voting Escrow Multiplication Error 12)
    VotingEscrowMultiplicationError12 = 11366,
    /// (Voting Escrow Addition Error 1)
    VotingEscrowAdditionError1 = 11367,
    /// (Voting Escrow Addition Error 2)
    VotingEscrowAdditionError2 = 11368,
    /// (Voting Escrow Addition Error 3)
    VotingEscrowAdditionError3 = 11369,
    /// (Voting Escrow Addition Error 5)
    VotingEscrowAdditionError5 = 11370,
    /// (Voting Escrow Addition Error 6)
    VotingEscrowAdditionError6 = 11371,
    /// (Voting Escrow Addition Error 7)
    VotingEscrowAdditionError7 = 11372,
    /// (Voting Escrow Addition Error 8)
    VotingEscrowAdditionError8 = 11373,
    /// (Voting Escrow Addition Error 9)
    VotingEscrowAdditionError9 = 11374,
    /// (Voting Escrow Addition Error 10)
    VotingEscrowAdditionError10 = 11375,
    /// (Voting Escrow Addition Error 11)
    VotingEscrowAdditionError11 = 11376,
    /// (Voting Escrow Addition Error 12)
    VotingEscrowAdditionError12 = 11377,
    /// (Voting Escrow Addition Error 13)
    VotingEscrowAdditionError13 = 11378,
    /// (Voting Escrow Addition Error 14)
    VotingEscrowAdditionError14 = 11379,
    /// (Voting Escrow Addition Error 15)
    VotingEscrowAdditionError15 = 11380,
    /// (Voting Escrow Addition Error 16)
    VotingEscrowAdditionError16 = 11381,
    /// (Voting Escrow Addition Error 17)
    VotingEscrowAdditionError17 = 11382,
    /// (Voting Escrow Addition Error 18)
    VotingEscrowAdditionError18 = 11383,
    /// (Voting Escrow Addition Error 19)
    VotingEscrowAdditionError19 = 11384,
    /// (Voting Escrow Addition Error 20)
    VotingEscrowAdditionError20 = 11385,
    /// (Voting Escrow Addition Error 21)
    VotingEscrowAdditionError21 = 11386,
    /// (Voting Escrow Addition Error 22)
    VotingEscrowAdditionError22 = 11387,
    /// (Voting Escrow Addition Error 23)
    VotingEscrowAdditionError23 = 11388,
    /// (Voting Escrow Addition Error 24)
    VotingEscrowAdditionError24 = 11389,
    /// (Voting Escrow Addition Error 25)
    VotingEscrowAdditionError25 = 11390,
    /// (Voting Escrow Addition Error 26)
    VotingEscrowAdditionError26 = 11391,
    /// (Voting Escrow Addition Error 27)
    VotingEscrowAdditionError27 = 11392,
    /// (Voting Escrow Subtraction Error 29)
    VotingEscrowSubtractionError29 = 11393,
    /// (Voting Escrow Subtraction Error 30)
    VotingEscrowSubtractionError30 = 11394,
    /// (Voting Escrow Subtraction Error 31)
    VotingEscrowSubtractionError31 = 11395,
    /// (Voting Escrow Subtraction Error 32)
    VotingEscrowSubtractionError32 = 11396,
    /// (Voting Escrow Subtraction Error 33)
    VotingEscrowSubtractionError33 = 11397,
    /// (Voting Escrow Subtraction Error 34)
    VotingEscrowSubtractionError34 = 11398,
    /// (Voting Escrow Subtraction Error 35)
    VotingEscrowSubtractionError35 = 11399,
    /// (Voting Escrow Division Error 10)
    VotingEscrowDivisionError10 = 11400,
    /// (Voting Escrow Division Error 11)
    VotingEscrowDivisionError11 = 11401,
    /// (Voting Escrow Division Error 12)
    VotingEscrowDivisionError12 = 11402,
    /// (Voting Escrow Multiplication Error 15)
    VotingEscrowMultiplicationError15 = 11403,
    /// (Voting Escrow Multiplication Error 16)
    VotingEscrowMultiplicationError16 = 11404,
    /// (Voting Escrow Not Controller)
    VotingEscrowNotController = 11405,
    /// (Voting Escrow Invalid Block Number1)
    VotingEscrowInvalidBlockNumber1 = 11406,
    /// (Voting Escrow Invalid Block Number2)
    VotingEscrowInvalidBlockNumber2 = 11407,

    /// (Liquidity Gauge Wrapper Unauthorized)
    GaugeWrapperUnauthorized = 11501,
    /// (Liquidity Gauge Wrapper Is Killed)
    GaugeWrapperIsKilled1 = 11502,
    /// (Liquidity Gauge Wrapper Not Approved)
    GaugeWrapperNotApproved = 11503,
    /// (Liquidity Gauge Wrapper IsLocked1)
    GaugeWrapperIsLocked1 = 11504,
    /// (Liquidity Gauge Wrapper IsLocked2)
    GaugeWrapperIsLocked2 = 11505,
    /// (Liquidity Gauge Wrapper Is Locked3)
    GaugeWrapperIsLocked3 = 11506,
    /// (Liquidity Gauge Wrapper Admin Only1)
    GaugeWrapperAdminOnly1 = 11507,
    /// (Liquidity Gauge Wrapper Admin Only2)
    GaugeWrapperAdminOnly2 = 11508,
    /// (Liquidity Gauge Wrapper Admin Only3)
    GaugeWrapperAdminOnly3 = 11509,
    /// (Liquidity Gauge Wrapper Admin Not Set)
    GaugeWrapperAdminNotSet = 11510,
    /// (Liquidity Gauge Wrapper Is Killed)
    GaugeWrapperIsKilled2 = 11511,
    /// (Liquidity Gauge Wrapper Addition Error 1)
    GaugeWrapperAdditionError1 = 11512,
    /// (Liquidity Gauge Wrapper Addition Error 2)
    GaugeWrapperAdditionError2 = 11513,
    /// (Liquidity Gauge Wrapper Addition Error 3)
    GaugeWrapperAdditionError3 = 11514,
    /// (Liquidity Gauge Wrapper Addition Error 4)
    GaugeWrapperAdditionError4 = 11515,
    /// (Liquidity Gauge Wrapper Addition Error 5)
    GaugeWrapperAdditionError5 = 11516,
    /// (Liquidity Gauge Wrapper Addition Error 6)
    GaugeWrapperAdditionError6 = 11517,
    /// (Liquidity Gauge Wrapper Addition Error 7)
    GaugeWrapperAdditionError7 = 11518,
    /// (Liquidity Gauge Wrapper Subtraction Error 1)
    GaugeWrapperSubtractionError1 = 11519,
    /// (Liquidity Gauge Wrapper Subtraction Error 2)
    GaugeWrapperSubtractionError2 = 11520,
    /// (Liquidity Gauge Wrapper Subtraction Error 3)
    GaugeWrapperSubtractionError3 = 11521,
    /// (Liquidity Gauge Wrapper Subtraction Error 4)
    GaugeWrapperSubtractionError4 = 11522,
    /// (Liquidity Gauge Wrapper Subtraction Error 5)
    GaugeWrapperSubtractionError5 = 11523,
    /// (Liquidity Gauge Wrapper Subtraction Error 6)
    GaugeWrapperSubtractionError6 = 11524,
    /// (Liquidity Gauge Wrapper Subtraction Error 7)
    GaugeWrapperSubtractionError7 = 11525,
    /// (Liquidity Gauge Wrapper Division Error 1)
    GaugeWrapperDivisionError1 = 11526,
    /// (Liquidity Gauge Wrapper Division Error 2)
    GaugeWrapperDivisionError2 = 11527,
    /// (Liquidity Gauge Wrapper Division Error 3)
    GaugeWrapperDivisionError3 = 11528,
    /// (Liquidity Gauge Wrapper Division Error 4)
    GaugeWrapperDivisionError4 = 11529,
    /// (Liquidity Gauge Wrapper Multiply Error 1)
    GaugeWrapperMultiplyError1 = 11530,
    /// (Liquidity Gauge Wrapper Multiply Error 2)
    GaugeWrapperMultiplyError2 = 11531,
    /// (Liquidity Gauge Wrapper Multiply Error 3)
    GaugeWrapperMultiplyError3 = 11532,
    /// (Liquidity Gauge Wrapper Multiply Error 4)
    GaugeWrapperMultiplyError4 = 11533,

    /// (Ownable: caller is not the owner)
    OwnableNotOwner = 11601,
    /// (Ownable: new owner is the zero address)
    OwnableNewOwnerAddressZero = 11602,

    /// (IRewardDistributionRecipient: Caller is not reward distribution)
    NotRewardDistribution = 11701,

    /// (CurveRewards:Cannot stake 0)
    CurveRewardsCannotStake = 11801,
    /// (CurveRewards:Cannot withdraw 0)
    CurveRewardsCannotWithdraw = 11802,
    /// (CurveRewards:Addition Error 1)
    CurveRewardsAdditionError1 = 11803,
    /// (CurveRewards:Addition Error 2)
    CurveRewardsAdditionError2 = 11804,
    /// (CurveRewards:Addition Error 3)
    CurveRewardsAdditionError3 = 11805,
    /// (CurveRewards:Addition Error 4)
    CurveRewardsAdditionError4 = 11806,
    /// (CurveRewards:Division Error 1)
    CurveRewardsDivisionError1 = 11807,
    /// (CurveRewards:Division Error 2)
    CurveRewardsDivisionError2 = 11808,
    /// (CurveRewards:Division Error 3)
    CurveRewardsDivisionError3 = 11809,
    /// (CurveRewards:Division Error 4)
    CurveRewardsDivisionError4 = 11810,
    /// (CurveRewards:Subtraction Error 1)
    CurveRewardsSubtractionError1 = 11811,
    /// (CurveRewards:Subtraction Error 2)
    CurveRewardsSubtractionError2 = 11812,
    /// (CurveRewards:Subtraction Error 3)
    CurveRewardsSubtractionError3 = 11813,
    /// (CurveRewards:Multiply Error 1)
    CurveRewardsMultiplyError1 = 11814,
    /// (CurveRewards:Multiply Error 2)
    CurveRewardsMultiplyError2 = 11815,
    /// (CurveRewards:Multiply Error 3)
    CurveRewardsMultiplyError3 = 11816,
    /// (CurveRewards:Multiply Error 4)
    CurveRewardsMultiplyError4 = 11817,

    //LIQUIDITY GUAGE V3
    /// (Liquidity guage v3 OverFlow1)
    LiquidityGaugeOverFlow1 = 11901,
    /// (Liquidity guage v3 OverFlow2)
    LiquidityGaugeOverFlow2 = 11902,
    /// (Liquidity guage v3 OverFlow3)
    LiquidityGaugeOverFlow3 = 11903,
    /// (Liquidity guage v3 UnderFlow1)
    LiquidityGaugeUnderFlow1 = 11904,
    /// (Liquidity guage v3 UnderFlow2)
    LiquidityGaugeUnderFlow2 = 11905,
    ///(Liquidity guage v3 UnderFlow3)
    LiquidityGaugeUnderFlow3 = 11906,
    /// (Liquidity guage v3 UnderFlow4)
    LiquidityGaugeUnderFlow4 = 11907,
    /// (Liquidity guage v3 Underflow 5)
    LiquidityGaugeUnderFlow5 = 11908,
    /// (Liquidity guage v3 Underflow 6)
    LiquidityGaugeUnderFlow6 = 11909,
    /// (Liquidity guage v3 Only Admin1)
    LiquidityGaugeOnlyAdmin1 = 11910,
    /// (Liquidity guage v3 Only Admin2)
    LiquidityGaugeOnlyAdmin2 = 11911,
    /// (Liquidity guage v3 Only Admin3)
    LiquidityGaugeOnlyAdmin3 = 11912,
    /// (Liquidity guage v3 Only Future Admin)
    LiquidityGaugeOnlyFutureAdmin = 11913,
    /// (Liquidity guage v3 Cannot Redirect When Claiming For Another User)
    LiquidityGaugeCannotRedirectWhenClaimingForAnotherUser = 11914,
    /// (Liquidity guage v3 Cannot Modify Existing Reward Token)
    LiquidityGaugeCannotModifyExistingRewardToken = 11915,
    /// (Liquidity guage locked 1)
    LiquidityGaugeLocked1 = 11916,
    /// (Liquidity guage v3 locked 2)
    LiquidityGaugeLocked2 = 11917,
    /// (Liquidity guage v3 locked 3)
    LiquidityGaugeLocked3 = 11918,
    /// (Liquidity guage v3 locked 4)
    LiquidityGaugeLocked4 = 11919,
    /// (Liquidity guage v3 locked 5)
    LiquidityGaugeLocked5 = 11920,
    /// (Liquidity guage v3 locked 6)
    LiquidityGaugeLocked6 = 11921,

    /// (Liquidity guage v3 unauthorized)
    LiquidityGuageUnauthorized = 11922,

    /// (Liquidity guage v3 Kick not allowed1)
    LiquidityGuageKickNotAllowed1 = 11923,
    /// (Liquidity guage v3 Kick not allowed2)
    LiquidityGuageKickNotAllowed2 = 11924,
    /// (Liquidity guage v3 TokenIsZeroAddress)
    LiquidityGaugeTokenIsZeroAddress = 11925,
    /// (Liquidity guage v3 zero total supply)
    LiquidityGaugeZeroTotalSupply = 11926,
    /// (Liquidity guage v3 failed deposit)
    LiquidityGaugeFailedToDeposit = 11927,
    /// (Liquidity guage v3 failed to withdraw)
    LiquidityGaugeFailedToWithdraw = 11928,
    /// (Liquidity guage v3 Arithmetic error 1)
    LiquidityGaugeArithmeticError1 = 11929,
    /// (Liquidity guage v3 Arithmetic error 2)
    LiquidityGaugeArithmeticError2 = 11930,
    /// (Liquidity guage v3 Arithmetic error 3)
    LiquidityGaugeArithmeticError3 = 11931,
    /// (Liquidity guage v3 Arithmetic error 4)
    LiquidityGaugeArithmeticError4 = 11932,
    /// (Liquidity guage v3 Arithmetic error 5)
    LiquidityGaugeArithmeticError5 = 11933,
    /// (Liquidity guage v3 Arithmetic error 6)
    LiquidityGaugeArithmeticError6 = 11934,
    /// (Liquidity guage v3 Arithmetic error 7)
    LiquidityGaugeArithmeticError7 = 11935,
    /// (Liquidity guage v3 Arithmetic error 8)
    LiquidityGaugeArithmeticError8 = 11936,
    /// (Liquidity guage v3 Arithmetic error 9)
    LiquidityGaugeArithmeticError9 = 11937,
    /// (Liquidity guage v3 Arithmetic error 10)
    LiquidityGaugeArithmeticError10 = 11938,
    /// (Liquidity guage v3 Arithmetic error 11)
    LiquidityGaugeArithmeticError11 = 11939,
    /// (Liquidity guage v3 Arithmetic error 12)
    LiquidityGaugeArithmeticError12 = 11940,
    /// (Liquidity guage v3 Arithmetic error 13)
    LiquidityGaugeArithmeticError13 = 11941,
    /// (Liquidity guage v3 Arithmetic error 14)
    LiquidityGaugeArithmeticError14 = 11942,
    /// (Liquidity guage v3 Arithmetic error 15)
    LiquidityGaugeArithmeticError15 = 11943,
    /// (Liquidity guage v3 Arithmetic error 16)
    LiquidityGaugeArithmeticError16 = 11944,
    /// (Liquidity guage v3 Arithmetic error 17)
    LiquidityGaugeArithmeticError17 = 11945,
    /// (Liquidity guage v3 Arithmetic error 18)
    LiquidityGaugeArithmeticError18 = 11946,
    /// (Liquidity guage v3 Arithmetic error 19)
    LiquidityGaugeArithmeticError19 = 11947,
    /// (Liquidity guage v3 Arithmetic error 20)
    LiquidityGaugeArithmeticError20 = 11948,
    /// (Liquidity guage v3 Arithmetic error 21)
    LiquidityGaugeArithmeticError21 = 11949,
    /// (Liquidity guage v3 Arithmetic error 22)
    LiquidityGaugeArithmeticError22 = 11950,
    /// (Liquidity guage v3 Arithmetic error 23)
    LiquidityGaugeArithmeticError23 = 11951,
    /// (Liquidity guage v3 Arithmetic error 24)
    LiquidityGaugeArithmeticError24 = 11952,
    /// (Liquidity guage v3 Arithmetic error 25)
    LiquidityGaugeArithmeticError25 = 11953,
    /// (Liquidity guage v3 Arithmetic error 26)
    LiquidityGaugeArithmeticError26 = 11954,
    /// (Liquidity guage v3 Arithmetic error 27)
    LiquidityGaugeArithmeticError27 = 11955,
    /// (Liquidity guage v3 Arithmetic error 28)
    LiquidityGaugeArithmeticError28 = 11956,
    /// (Liquidity guage v3 Arithmetic error 29)
    LiquidityGaugeArithmeticError29 = 11957,
    /// (Liquidity guage v3 Arithmetic error 30)
    LiquidityGaugeArithmeticError30 = 11958,
    /// (Liquidity guage v3 Arithmetic error 31)
    LiquidityGaugeArithmeticError31 = 11959,
    /// (Liquidity guage v3 Arithmetic error 32)
    LiquidityGaugeArithmeticError32 = 11960,
    /// (Liquidity guage v3 Arithmetic error 33)
    LiquidityGaugeArithmeticError33 = 11961,
    /// (Liquidity guage v3 Arithmetic error 34)
    LiquidityGaugeArithmeticError34 = 11962,
    /// (Liquidity guage v3 Arithmetic error 35)
    LiquidityGaugeArithmeticError35 = 11963,
    /// (Liquidity guage v3 Arithmetic error 36)
    LiquidityGaugeArithmeticError36 = 11964,
    /// (Liquidity guage v3 Arithmetic error 37)
    LiquidityGaugeArithmeticError37 = 11965,
    /// (Liquidity guage v3 Arithmetic error 38)
    LiquidityGaugeArithmeticError38 = 11966,
    /// (Liquidity guage v3 Arithmetic error 39)
    LiquidityGaugeArithmeticError39 = 11967,
    /// (Liquidity guage v3 Arithmetic error 40)
    LiquidityGaugeArithmeticError40 = 11968,
    /// (Liquidity guage v3 Arithmetic error 41)
    LiquidityGaugeArithmeticError41 = 11969,
    /// (Liquidity guage v3 Arithmetic error 42)
    LiquidityGaugeArithmeticError42 = 11970,
    /// (Liquidity guage v3 Arithmetic error 43)
    LiquidityGaugeArithmeticError43 = 11971,
    /// (Liquidity guage v3 Arithmetic error 44)
    LiquidityGaugeArithmeticError44 = 11972,
    /// (Liquidity guage v3 Arithmetic error 45)
    LiquidityGaugeArithmeticError45 = 11973,
    /// (Liquidity guage v3 Arithmetic error 46)
    LiquidityGaugeArithmeticError46 = 11974,
    /// (Liquidity guage v3 Arithmetic error 47)
    LiquidityGaugeArithmeticError47 = 11975,

    /// (Gauge Proxy Access Denied)
    GaugeProxyAccessDenied1 = 11976,
    /// (Gauge Proxy Access Denied)
    GaugeProxyAccessDenied2 = 11977,
    /// (Gauge Proxy Access Denied)
    GaugeProxyAccessDenied3 = 11978,
    /// (Gauge Proxy Access Denied)
    GaugeProxyAccessDenied4 = 11979,
    /// (Gauge Proxy Access Denied)
    GaugeProxyAccessDenied5 = 11980,
    /// (Gauge Proxy Is Locked)
    GaugeProxyIsLocked1 = 11981,
    /// (Gauge Proxy Is Locked)
    GaugeProxyIsLocked2 = 11982,
    /// (Gauge Proxy Is Locked)
    GaugeProxyIsLocked3 = 11983,

    //LIQUIDITY GUAGE V3
    /// (Liquidity Gauge V4 error 1)
    LiquidityGaugeV4RewardCountExceedsMax = 11984,
    /// (Liquidity Gauge V4 error 2)
    LiquidityGaugeV4RewardDataExists1 = 11985,
    /// (Liquidity Gauge V4 error 3)
    LiquidityGaugeV4AddRewardAdditionOverFlow = 11986,
    /// (Liquidity Gauge V4 error 4)
    LiquidityGaugeV4CurrentDistributorExists1 = 11987,
    /// (Liquidity Gauge V4 error 5)
    LiquidityGaugeV4NewDistributorExists1 = 11988,
    /// (Liquidity Gauge V4 error 6)
    LiquidityGaugeOnlyAdminOrDistributor = 11989,
    /// (Liquidity Gauge V4 error 7)
    LiquidityGaugeOnlyDistributor = 11990,
    /// (Liquidity Gauge V4 error 8)
    LiquidityGaugeV4DepositRewardTokensSubtractionOverFlow = 11991,
    /// (Liquidity Gauge V4 error 9)
    LiquidityGaugeV4DepositRewardTokensMultiplicationOverFlow = 11992,    
    /// (Liquidity Gauge V4 error 10)
    LiquidityGaugeV4DepositRewardTokensAdditionOverFlow1 = 11993,
    /// (Liquidity Gauge V4 error 11)
    LiquidityGaugeV4DepositCheckpointRewardsSubtractionOverFlow = 11994,
    /// (Liquidity Gauge V4 error 12)
    LiquidityGaugeV4DepositCheckpointRewardsMultiplicationOverFlow1 = 11995,
    /// (Liquidity Gauge V4 error 13)
    LiquidityGaugeV4DepositCheckpointRewardsMultiplicationOverFlow2 = 11996,
    /// (Liquidity Gauge V4 error 14)
    LiquidityGaugeV4DepositCheckpointRewardsAdditionOverFlow = 11997,
    /// (Liquidity Gauge V4 error 15)
    LiquidityGaugeV4DepositRewardTokensAdditionOverFlow2 = 11998,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}
