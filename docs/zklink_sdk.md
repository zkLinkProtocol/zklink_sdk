# crypto
密码学基础模块，包含公私钥生成、哈希计算、签名等基础功能
## privateKeyFromSeed
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| seed  | 字节数组 | 随机种子 |
### process
通过seed进行sha256哈希生成二层私钥
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| privateKey  | 字节数组 | 二层私钥 |
## signTransactionBytes
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| privateKey  | 字节数组 | 二层私钥 |
| txBytes  | 字节数组 | 待签名的交易序列化数据 |
### process
使用musig Schnorr签名方案对交易数据进行签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| pubKey  | 字符串 | signer对应的公钥 |
| signature  | 字符串 | 生成的签名 |
## privateKeyToPubKey
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| privateKey  | 字节数组 | 二层私钥 |
### process
生成私钥对应的公钥
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| pubKey  | 字符串 | 私钥对应的公钥 |
## privateKeyToPubKeyHash
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| privateKey  | 字节数组 | 二层私钥 |
### process
生成私钥对应的公钥哈希
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| pubKey  | 字符串 | 私钥对应的公钥哈希 |
## rescueHashOrders
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| orders  | 字节数组 | 序列化的order数据 |
### process
生成orders订单请求的rescure哈希
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| hash  | 字节数组 | 生成的哈希字节数组 |
# eth-message-signer
对二层交易请求生成L1签名，两种签名类型，ECDSA和EIP1271
## ethSignTransfer
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| stringAmount  | 字符串 | transfer的金额 |
| stringToken  | 字符串 | token的address |
| stringFee  | 字符串 | 手续费 |
| to  | 字符串 | 目的地址 |
| nonce  | 整数 | from账户的2层nonce |
| accountId  | 整数 | from账户的2层id |
### process
生成transfer请求的L1签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 枚举 | 签名类型ECDSA或EIP1271 |
| signature  | 字符串 | 生成的L1签名 |
## ethSignOrderMatching
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| stringFeeToken  | 字符串 | feeToken地址 |
| stringFee  | 字符串 | 手续费 |
### process
生成OrderMathing请求的L1签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 枚举 | 签名类型ECDSA或EIP1271 |
| signature  | 字符串 | 生成的L1签名 |
## ethSignOrder
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'Order' |
| accountId  | number | 2层账户id |
| subAccountId  | number | 子账户id |
| slotId  | number | 槽位号 |
| nonce  | number | 2层账户id |
| baseTokenId  | number | base代币的id |
| quoteTokenId  | number | quote代币的id |
| amount  | 大整数 | 挂单数量 |
| price  | 大整数 | 挂单价格 |
| isSell  | number | 是否是卖单 |
| feeRatio1  | number | 卖单手续费率 |
| feeRatio2  | number | 买单手续费率 |
| signature  | 字符串 | 订单签名 |
| address  | 字符串 | 挂单地址 |
| stringPrice  | 字符串 | 挂单价格 |
| stringAmount  | 字符串 | 挂单金额 |
| baseTokenSymbol  | 字符串 | base代币的symbol |
| quoteTokenSymbol  | 字符串 | quote代币的symbol |
### process
生成订单的L1签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 枚举 | 签名类型ECDSA或EIP1271 |
| signature  | 字符串 | 生成的L1签名 |
## ethSignCreatePool
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| token0  | 字符串 | pool的token0地址 |
| token1  | 字符串 | pool的token1地址 |
| nonce  | 整数 | from账户的2层nonce |
| accountId  | 整数 | from账户的2层id |
### process
生成createPool请求的L1签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 枚举 | 签名类型ECDSA或EIP1271 |
| signature  | 字符串 | 生成的L1签名 |
## ethSignForcedExit
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| stringToken  | 字符串 | token的address |
| stringFeeToken  | 字符串 | 手续费token地址 |
| stringFee  | 字符串 | 手续费 |
| target  | 字符串 | forceExit提币目的地址 |
| nonce  | 整数 | from账户的2层nonce |
### process
生成forceExit请求的L1签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 枚举 | 签名类型ECDSA或EIP1271 |
| signature  | 字符串 | 生成的L1签名 |
## ethSignWithdraw
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| stringAmount  | 字符串 | transfer的金额 |
| stringToken  | 字符串 | token的address |
| stringFee  | 字符串 | 手续费 |
| to  | 字符串 | 目的地址 |
| nonce  | 整数 | from账户的2层nonce |
| accountId  | 整数 | from账户的2层id |
### process
生成withdraw请求的L1签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 枚举 | 签名类型ECDSA或EIP1271 |
| signature  | 字符串 | 生成的L1签名 |
# signer
对交易生成L2签名
## signTransfer
### data struct
#### Signatrue
```
Signature {
  pubKey: string
  signature: string
}
```
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'Transfer' |
| accountId  | number | 2层账户id |
| fromSubAccountId  | number | from子账户id |
| toSubAccountId  | number | to子账户id |
| from  | address | from地址 |
| to  | address | to地址 |
| token  | number | token id |
| amount  | 大整数 | 转账金额 |
| fee  | 大整数 | 手续费 |
| ts  | number | timestamp |
| nonce  | number | from的2层nonce |
| signature  | Signatrue | 空 |
### process
生成transfer交易请求的L2签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'Transfer' |
| accountId  | number | 2层账户id |
| fromSubAccountId  | number | from子账户id |
| toSubAccountId  | number | to子账户id |
| from  | address | from地址 |
| to  | address | to地址 |
| token  | number | token id |
| amount  | 大整数 | 转账金额 |
| fee  | 大整数 | 手续费 |
| ts  | number | timestamp |
| nonce  | number | from的2层nonce |
| signature  | Signatrue | 生成的L2签名 |
## signOrderMatching
### data struct
参照signOrder接口的input
```
OrderData {
  type: 'Order'
  accountId: number
  subAccountId: number
  slotId: number
  nonce: number
  baseTokenId: TokenId
  quoteTokenId: TokenId
  amount: BigNumberish
  price: BigNumberish
  isSell: number
  feeRatio1: number // be used for make
  feeRatio2: number // be used for taker
  signature?: Signature
}
```
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'OrderMatching' |
| accountId  | number | 2层账户id |
| subAccountId  | number | 子账户id |
| account  | Address | submitter |
| taker  | OrderData | 买单数据 |
| maker  | OrderData | 卖单数据 |
| expectBaseAmount  | number | token id |
| expectQuoteAmount  | BigNumberish | 转账金额 |
| fee  | BigNumberish | 手续费 |
| feeToken  | number | timestamp |
| nonce  | number | from的2层nonce |
| signature  | Signatrue | 空 |
### process
生成撮合订单交易的L2签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'OrderMatching' |
| accountId  | number | 2层账户id |
| subAccountId  | number | 子账户id |
| account  | Address | submitter |
| taker  | OrderData | 买单数据 |
| maker  | OrderData | 卖单数据 |
| expectBaseAmount  | number | token id |
| expectQuoteAmount  | BigNumberish | 转账金额 |
| fee  | BigNumberish | 手续费 |
| feeToken  | number | timestamp |
| nonce  | number | from的2层nonce |
| signature  | Signatrue | 生成的L2签名 |
## signOrder
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'Order' |
| accountId  | number | 挂单2层账户id |
| subAccountId  | number | 子账户id |
| account  | Address | submitter |
| slotId  | number | 槽位 |
| nonce  | number | 挂单账户的nonce |
| baseTokenId  | number | base token id |
| quoteTokenId  | number | quote token id |
| amount  | BigNumberish | 挂单金额 |
| price  | BigNumberish | 挂单 |
| isSell  | number | 卖单标志 |
| feeRatio1  | number | 卖单费率 |
| feeRatio2  | number | 买单费率 |
| signature  | Signatrue | 空 |
### process
生成撮合订单交易的L2签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'Order' |
| accountId  | number | 挂单2层账户id |
| subAccountId  | number | 子账户id |
| account  | Address | submitter |
| slotId  | number | 槽位 |
| nonce  | number | 挂单账户的nonce |
| baseTokenId  | number | base token id |
| quoteTokenId  | number | quote token id |
| amount  | BigNumberish | 挂单金额 |
| price  | BigNumberish | 挂单 |
| isSell  | number | 卖单标志 |
| feeRatio1  | number | 卖单费率 |
| feeRatio2  | number | 买单费率 |
| signature  | Signatrue | 生成的L2签名 |
## signWithdraw
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'Withdraw' |
| toChainId  | number | 提现的目标链2层id |
| accountId  | number | 挂单2层账户id |
| subAccountId  | number | 子账户id |
| from  | Address | from账户地址 |
| to  | Address | to账户地址 |
| l2SourceToken  | number | 要提现的2层源tokenId |
| l1TargetToken  | number | 提现的1层目的tokenId |
| amount  | BigNumberish | 提现金额 |
| fee  | BigNumberish | 手续费 |
| withdrawFeeRatio  | number | 提现费率 |
| fastWithdraw  | number | 是否快提 |
| ts  | number | 时间戳 |
| nonce  | number | from账户的2层nonce |
| signature  | Signatrue | 空 |
### process
生成提现交易的L2签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'Withdraw' |
| toChainId  | number | 提现的目标链2层id |
| accountId  | number | 挂单2层账户id |
| subAccountId  | number | 子账户id |
| from  | Address | from账户地址 |
| to  | Address | to账户地址 |
| l2SourceToken  | number | 要提现的2层源tokenId |
| l1TargetToken  | number | 提现的1层目的tokenId |
| amount  | BigNumberish | 提现金额 |
| fee  | BigNumberish | 手续费 |
| withdrawFeeRatio  | number | 提现费率 |
| fastWithdraw  | number | 是否快提 |
| ts  | number | 时间戳 |
| nonce  | number | from账户的2层nonce |
| signature  | Signatrue | 空 |
## signForcedExit
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'ForcedExit' |
| toChainId  | number | 目标链2层id |
| initiatorAccountId  | number | 2层账户id |
| initiatorSubAccountId  | number | 子账户id |
| target  | Address | 目标地址 |
| targetSubAccountId  | number | 目标子账户id |
| l2SourceToken  | number | 要退出的2层源tokenId |
| l1TargetToken  | number | 退出的1层目的tokenId |
| initiatorNonce  | number | initiator的2层nonce |
| exitAmount  | BigNumberish | 退出金额 |
| ts  | number | 时间戳 |
| signature  | Signatrue | 空 |
### process
生成强制退出交易的L2签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'ForcedExit' |
| toChainId  | number | 目标链2层id |
| initiatorAccountId  | number | 2层账户id |
| initiatorSubAccountId  | number | 子账户id |
| target  | Address | 目标地址 |
| targetSubAccountId  | number | 目标子账户id |
| l2SourceToken  | number | 要退出的2层源tokenId |
| l1TargetToken  | number | 退出的1层目的tokenId |
| initiatorNonce  | number | initiator的2层nonce |
| exitAmount  | BigNumberish | 退出金额 |
| ts  | number | 时间戳 |
| signature  | Signatrue | 空 |
## signChangePubKey
### data struct
ChangePubKeyAuthData
```
export interface ChangePubKeyOnchain {
  type: 'Onchain'
}

export interface ChangePubKeyECDSA {
  type: 'EthECDSA'
  ethSignature: string
  batchHash?: string
}

export interface ChangePubKeyCREATE2 {
  type: 'EthCREATE2'
  creatorAddress: string
  saltArg: string
  codeHash: string
}
```
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'ChangePubKey' |
| chainId  | number | 2层chain id |
| accountId  | number | 2层账户id |
| subAccountId  | number | 子账户id |
| account  | Address | 账户地址 |
| newPkHash  | PubKeyHash | 新的pubkey哈希 |
| feeToken  | number | 手续费token id |
| fee  | BigNumberish | 手续费数量 |
| ts  | number | 时间戳 |
| nonce  | number | 账户的2层nonce |
| signature  | Signatrue | 空 |
| ethAuthData  | 枚举ChangePubKeyAuthData | 根据不同changePubkey类型填充相应的验签数据 |

### process
生成change_pubkey交易的L2签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| type  | 字符串 | 'ChangePubKey' |
| chainId  | number | 2层chain id |
| accountId  | number | 2层账户id |
| subAccountId  | number | 子账户id |
| account  | Address | 账户地址 |
| newPkHash  | PubKeyHash | 新的pubkey哈希 |
| feeToken  | number | 手续费token id |
| fee  | BigNumberish | 手续费数量 |
| ts  | number | 时间戳 |
| nonce  | number | 账户的2层nonce |
| signature  | Signatrue | 生成的L2签名 |
| ethAuthData  | 枚举ChangePubKeyAuthData | 根据不同changePubkey类型填充相应的验签数据 |
# wallet
钱包交互的关键模块，包含钱包创建、签名交易、提交交易功能
## fromEthSigner
### data struct
```
export type EthSignerType = {
  verificationMethod: 'ECDSA' | 'ERC-1271'
  // Indicates if signer adds `\x19Ethereum Signed Message\n${msg.length}` prefix before signing message.
  // i.e. if false, we should add this prefix manually before asking to sign message
  isSignedMsgPrefixed: boolean
}
export class Wallet {
    public ethSigner: ethers.Signer,
    public ethMessageSigner: EthMessageSigner,
    public cachedAddress: Address,
    public signer?: Signer,
    public accountId?: number,
    public ethSignerType?: EthSignerType
  }
```
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| ethWallet  | ethers.Signer | L1钱包对象 |
| signer  | Signer | L2钱包对象，可选 |
| accountId  | number | 2层账户id，可选 |
| ethSignerType  | EthSignerType | 钱包验签类型 |
### process
从1层钱包创建2层钱包对象
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| wallet  | Wallet | L2钱包对象 |
## fromCreate2Data
### data struct
```
export interface Create2Data {
  creatorAddress: string
  saltArg: string
  codeHash: string
}
```
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| createrSigner  | ethers.Signer | L1钱包对象 |
| syncSigner  | Signer | L2钱包对象，可选 |
| accountId  | number | 2层账户id，可选 |
| create2Data  | Create2Data | create2数据 |
### process
从create2合约账户创建2层钱包对象
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| wallet  | Wallet | L2钱包对象 |
## getEIP712Signature
### data struct
```
export interface TxEthSignature {
  type: 'EthereumSignature' | 'EIP1271Signature'
  signature: string
}
```
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| data  | any | 待签名数据 |
### process
将data按照eip712进行签名
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| signature  | TxEthSignature | 签名数据 |
## isOnchainAuthSigningKeySet
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| mainContract  | Address | zklink合约地址 |
| nonce  | number | 钱包账户2层nonce |
### process
从L1链上查询钱包账户是否在链上验证过
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| isAuth  | bool | 是否在链上验证过 |
## onchainAuthSigningKey
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| mainContract  | Address | zklink合约地址 |
| nonce  | number | 钱包账户2层nonce |
| currentPubKeyHash  | PubKeyHash | 当前的pubkeyhash |
| ethTxOptions  | ethers.providers.TransactionRequest | 可选 |
### process
非create2合约账户调用L1链上setAuthPubkeyHash进行pubkeyhash设置
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txHash  | ContractTransaction | 合约执行交易hash |
## sendDepositFromEthereum
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| mainContract  | Address | zklink合约地址 |
| subAccountId  | number | 子账户id |
| depositTo  | Address | 充值的目的地址 |
| token  | adress | 充值的token地址 |
| amount  | BigNumberish | 充值金额 |
| mapping  | boolean | 是否映射到usd |
| ethTxOptions  | ethers.providers.TransactionRequest | 可选 |
| approveDepositAmountForERC20  | boolean | 是否允许approve erc20最大数量，可选 |
### process
调用Deposit合约充值
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txResp  | TransactionResponse | Deposit交易执行回执 |
## emergencyWithdraw
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| mainContract  | Address | zklink合约地址 |
| subAccountId  | number | 子账户id |
| tokenId  | number | 2层tokenid |
| accountId  | number | 2层账户id |
| ethTxOptions  | ethers.providers.TransactionRequest | 可选 |
### process
调用FullExit将L2资金全部退出到用户账户
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txResp  | TransactionResponse | FullExit交易执行回执 |
## submit_change_pub_key
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| chainId  | number | 2层chain id |
| subAccountId  | number | 子账户id |
| newPkHash  | PubKeyHash | 新的pubkey哈希 |
| feeToken  | number | 手续费token id |
| fee  | BigNumberish | 手续费数量 |
| nonce  | number | 账户的2层nonce |
| ethAuthData  | 枚举ChangePubKeyAuthData | 根据不同changePubkey类型填充相应的验签数据 |
### process
提交change_pub_key交易
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txHash  | 字符串 | 2层交易hash |
## submit_transfer
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| from_sub_account_id  | number | from_sub_account_id |
| to  | Address | 目标地址 |
| to_sub_account_id  | number | to_sub_account_id |
| amount  | BigNumberish |  转账金额 |
| fee  | BigNumberish | 手续费 |
| nonce  | number | 账户的2层nonce |
### process
钱包提交transfer交易
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txHash  | 字符串 | 2层交易hash |
## submit_withdraw
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| toChainId  | number | 提现的目标链2层id |
| subAccountId  | number | 子账户id |
| to  | Address | to账户地址 |
| l2SourceToken  | number | 要提现的2层源tokenId |
| l1TargetToken  | number | 提现的1层目的tokenId |
| amount  | BigNumberish | 提现金额 |
| fee  | BigNumberish | 手续费 |
| withdrawFeeRatio  | number | 提现费率 |
| fastWithdraw  | bool | 是否快提 |
| nonce  | number | 账户的2层nonce |
### process
钱包提交withdraw交易
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txHash  | 字符串 | 2层交易hash |
## submit_forced_exit
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| toChainId  | number | 目标链2层id |
| subAccountId  | number | 子账户id |
| target  | Address | 目标地址 |
| targetSubAccountId  | number | 目标子账户id |
| l2SourceToken  | number | 要退出的2层源tokenId |
| l1TargetToken  | number | 退出的1层目的tokenId |
| nonce  | number | 账户2层nonce |
| exitAmount  | BigNumberish | 退出金额 |
### process
提交forceExit交易
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txHash  | 字符串 | 2层交易hash |
## submit_order_matching
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| subAccountId  | number | 子账户id |
| taker  | OrderData | 买单数据 |
| maker  | OrderData | 卖单数据 |
| expectBaseAmount  | number | token id |
| expectQuoteAmount  | BigNumberish | 转账金额 |
| fee  | BigNumberish | 手续费 |
| feeToken  | number | timestamp |
| nonce  | number | submitter的2层nonce |
### process
taker和maker的数据由挂单用户签名生成，参见[signOrder](#signorder)接口.提交OrderMatching交易(目前只有测试使用，看是否需要保留)
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txHash  | 字符串 | 2层交易hash |
# util
通用函数模块，包含交易的序列化、数据类型转换，以及钱包通用接口，例如账户余额查询、gas预估、erc20的approve等
## getEthereumBalance
### input
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| mainContract  | Address | zklink合约地址 |
| nonce  | number | 钱包账户2层nonce |
| currentPubKeyHash  | PubKeyHash | 当前的pubkeyhash |
| ethTxOptions  | ethers.providers.TransactionRequest | 可选 |
### process
非create2合约账户调用L1链上setAuthPubkeyHash进行pubkeyhash设置
### output
|  名称   | 类型  | 描述 |
|  ----  | ----  | --- |
| txHash  | ContractTransaction | 合约执行交易hash |
## estimateGasDeposit
## isERC20DepositsApproved
## approveERC20TokenDeposits
