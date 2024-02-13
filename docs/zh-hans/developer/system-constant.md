# 系统枚举值

### 系统状态值 Status

```
enum SystemStatus {
    Off,
    On,
}
```

### Witness 类型值 DataType

```
enum DataType {
    ActionData = 0,
    AccountCellData,
    AccountSaleCellData,
    AccountAuctionCellData,
    ProposalCellData,
    PreAccountCellData, // 5
    IncomeCellData,
    OfferCellData,
    SubAccount,
    SubAccountMintSign,
    ReverseRecord, // 10
    SubAccountPriceRule,
    SubAccountPreservedRule,
    DeviceKeyList,
    SubAccountRenewSign,
    ConfigCellAccount = 100,              // args: 0x64000000
    ConfigCellApply = 101,                // args: 0x65000000
    ConfigCellIncome = 103,               // args: 0x67000000
    ConfigCellMain,                       // args: 0x68000000
    ConfigCellPrice,                      // args: 0x69000000
    ConfigCellProposal,                   // args: 0x6a000000
    ConfigCellProfitRate,                 // args: 0x6b000000
    ConfigCellRecordKeyNamespace,         // args: 0x6c000000
    ConfigCellRelease,                    // args: 0x6d000000
    ConfigCellUnAvailableAccount,         // args: 0x6e000000
    ConfigCellSecondaryMarket,            // args: 0x6f000000
    ConfigCellReverseResolution,          // args: 0x70000000
    ConfigCellSubAccount,                 // args: 0x71000000
    ConfigCellSubAccountBetaList,         // args: 0x72000000
    ConfigCellSystemStatus,               // args: 0x73000000
    ConfigCellSMTNodeWhitelist,           // args: 0x74000000
    ConfigCellPreservedAccount00 = 10000, // args: 0x10270000
    ConfigCellPreservedAccount01,
    ConfigCellPreservedAccount02,
    ConfigCellPreservedAccount03,
    ConfigCellPreservedAccount04,
    ConfigCellPreservedAccount05,
    ConfigCellPreservedAccount06,
    ConfigCellPreservedAccount07,
    ConfigCellPreservedAccount08,
    ConfigCellPreservedAccount09,
    ConfigCellPreservedAccount10,
    ConfigCellPreservedAccount11,
    ConfigCellPreservedAccount12,
    ConfigCellPreservedAccount13,
    ConfigCellPreservedAccount14,
    ConfigCellPreservedAccount15,
    ConfigCellPreservedAccount16,
    ConfigCellPreservedAccount17,
    ConfigCellPreservedAccount18,
    ConfigCellPreservedAccount19,     // args: 0x23270000
    ConfigCellCharSetEmoji = 100000,  // args: 0xa0860100
    ConfigCellCharSetDigit = 100001,  // args: 0xa1860100
    ConfigCellCharSetEn = 100002,     // args: 0xa2860100
    ConfigCellCharSetZhHans = 100003, // args: 0xa3860100, not available yet
    ConfigCellCharSetZhHant = 100004, // args: 0xa4860100, not available yet
    ConfigCellCharSetJp,              // args: 0xa5860100
    ConfigCellCharSetKo,              // args: 0xa6860100
    ConfigCellCharSetRu,              // args: 0xa7860100
    ConfigCellCharSetTr,              // args: 0xa8860100
    ConfigCellCharSetTh,              // args: 0xa9860100
    ConfigCellCharSetVi,              // args: 0xaa860100
}
```

> ⚠️ 为了方便维护， ConfigCellXXX 的编号空间为 100 ~ 199999 之间，既 args >= 100 && args <= 199999 .

### 字符集枚举值 CharSet

```
enum CharSetType {
    Emoji,
    Digit,
    En,
    ZhHans,
    ZhHant,
    Ja,
    Ko,
    Ru,
    Tr,
    Th,
    Vi, // ⚠️ DO NOT Forget to update CHAR_SET_LENGTH at the same time.
}
```

> ⚠️ 为了方便维护， ConfigCellCharSetXXX 总是保持减去 100000 就等于下面 CharSet 常量的形式，所以可以直接采用下面的转换方法：
>
> `ConfigCellCharSetEmoji - 100000 = EMOJI`