type RPCTransaction struct {
     Id      int64             `json:"id"`
     JSONRpc string            `json:"jsonrpc"`
     Method  string            `json:"method"`
     Params  []json.RawMessage `json:"params"`
}

type Signature struct {
 PubKey    string `json:"pubKey"`
 Signature string `json:"signature"`
}

// ChangePubKeyFunc is generated
func ChangePubKeyFunc(ctx context.Context, req *DtoChangePubKey) (*DtoEmpty, error) {

 changePubKey := &zkl.ChangePubKeyParams{}

 ethPrivateKey := "f6cafe4c8a531d3a66fc2c9e23caea5cfd9e572d6d966eec4229bd78d3b27803"
 seed, err := zkl.GenerateLayer2PrivateKeySeed(ethPrivateKey)
 if err != nil {
  return nil, util.NewError(err.Error(), 400)
 }

 // create zks private key by seed
 zkpk, err := zkl.NewZkSignerFromSeed(seed)
 if err != nil {
  TraceDepth("ChangePubKeyFunc", 4)
  return nil, util.NewError(err.Error(), 400)
 }

 newPkHash := fmt.Sprintf("%s", zkpk.GetPublicKeyHash())
 fmt.Println("newPkHash:", newPkHash)
 changePubKey.Type = req.Type
 changePubKey.ChainId = uint8(req.ChainId)
 changePubKey.EthAuthData.Type = "EthECDSA"
 changePubKey.FeeToken = uint16(req.FeeToken)
 changePubKey.Fee = decimal.NewFromInt(0)
 changePubKey.Nonce = uint32(req.Nonce)
 changePubKey.AccountId = uint32(req.AccountId)
 changePubKey.SubAccountId = uint8(req.SubAccountId)
 changePubKey.Ts = uint32(req.Ts)
 changePubKey.Signature.PubKey = req.Zksign.PubKey
 changePubKey.Signature.Signature = req.Zksign.Signature
 changePubKey.NewPkHash = req.NewPkHash                         //newPkHash
 changePubKey.EthAuthData.Signature = req.EthAuthData.Signature //"0xafabda44618bac69a68dfec8b67ccedd1c77fc7f51e0d1a0cd1db99e8250fc0473fb3a4b0e60b2a5136c4807a47e2d7803d64777fbbbebc41a21297e5586deb31c"
 l := logger.NewStdLogger()
 changePubKeyBytes, err := changePubKey.GetBytes(1)
 if err != nil {
  TraceDepth("ChangePubKeyFunc", 4)
  return nil, util.NewError(err.Error(), 400)
 }
 l.Info("ChangePubKeyFunc", "HEX", hex.EncodeToString(changePubKeyBytes))

 txHash, _, err := changePubKey.CalcTxHash(1)
 if err != nil {
  TraceDepth("ChangePubKeyFunc", 4)
  return nil, util.NewError(err.Error(), 400)
 }
 fmt.Println("txHash:", txHash)
 h := sha256.New()
 h.Write(changePubKeyBytes)
 bs := h.Sum(nil)
 //sum := sha256.Sum256(changePubKeyBytes)
 //fmt.Println("txHash sha256:", hex.EncodeToString(bs))
 l.Info("ChangePubKeyFunc", "txHash sha256:", hex.EncodeToString(bs))
 signature, err := zkpk.Sign(bs)
 if err != nil {
  TraceDepth("ChangePubKeyFunc", 4)
  return nil, util.NewError(err.Error(), 400)
 }

 fmt.Println("signature:", signature.HexString())
 //changePubKey.EthSignature = signature.HexString()
 jsonPubkey, err := json.Marshal(changePubKey)

 submitter := &Signature{}
 submitter.PubKey = zkpk.GetPublicKey() //req.Zksign.PubKey
 submitter.Signature = signature.HexString()
 jsonSubmitter, err := json.Marshal(submitter)

 tx := RPCTransaction{
  Id:      1,
  JSONRpc: "2.0",
  Method:  "sendTransaction",
  Params: []json.RawMessage{
   jsonPubkey,
   nil,
   jsonSubmitter,
  },
 }
 JsonTx, err := json.Marshal(tx)
 l.Info("ChangePubKeyFunc", "JsonTx", string(JsonTx))

 res, err := post.PostJson("https://aws-gw-v2.zk.link/", JsonTx)
 l.Info("ChangePubKeyFunc", "res", res, "err", err)
 //http.Post()

 return &DtoEmpty{}, nil
}

type NonceResp struct {
 JsonRpC string `json:"jsonrpc"`
 Result  struct {
  AccountId        int64            `json:"id"`
  Address          string           `json:"address"`
  Nonce            int64            `json:"nonce"`
  PubHash          string           `json:"pubKeyHash"`
  SubAccountNonces map[string]int64 `json:"subAccountNonces"`
 } `json:"result"`
 ID int64 `json:"id"`
}

func getNonce() int64 {
 JsonTx := []byte(`{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "getAccount",
  "params": ["0x0cb2adb3653c0e952dc13d51e7946f8b633c71d3"]
   }`)
 res, err := post.PostJson("https://aws-gw-v2.zk.link/", JsonTx)
 fmt.Println(res, err)
 rsp := NonceResp{}
 err = json.Unmarshal([]byte(res), &rsp)
 fmt.Println(res, err)
 return rsp.Result.SubAccountNonces["4"]
}

type SubmitterSignature struct {
 PubKey    string `json:"pubKey"`
 Signature string `json:"signature"`
}

// FaucetLayer2 is generated
func FaucetLayer2(ctx context.Context, req *DtoReqFaucetLayer2) (*DtoRspFaucetLayer2, error) {
 ethPrivateKey := "f6cafe4c8a531d3a66fc2c9e23caea5cfd9e572d6d966eec4229bd78d3b27803"
 e18, err := decimal.NewFromString("1000000000000000000")
 ToAddress := strings.ToLower(req.ToAddress)
 Amount := req.Amount
 damount, err := decimal.NewFromString(Amount)
 seed, err := zkl.GenerateLayer2PrivateKeySeed(ethPrivateKey)

 // create zks private key by seed
 zkpk, err := zkl.NewZkSignerFromSeed(seed)
 transferParams := &zkl.TransferParams{}
 transferParams.Type = "Transfer"
 nonce := getNonce()

 //f6cafe4c8a531d3a66fc2c9e23caea5cfd9e572d6d966eec4229bd78d3b27803
 transferParams.AccountId = 8300
 transferParams.FromSubAccountId = 4
 transferParams.To = ToAddress
 transferParams.ToSubAccountId = 4
 transferParams.Token = 153
 //damount, err := decimal.NewFromString(Amount)
 dfee, err := decimal.NewFromString("10000000000")
 transferParams.Amount = damount
 transferParams.Fee = dfee
 transferParams.Nonce = uint32(nonce)
 transferParams.Ts = uint32(time.Now().Unix())
 transferParams.Signature.PubKey = zkpk.GetPublicKey() //req.Zksign.PubKey
 transferBytes, err := transferParams.GetBytes(1)
 h := sha256.New()
 h.Write(transferBytes)
 bs := h.Sum(nil)

 // fmt.Println("transferBytes1", hex.EncodeToString(transferBytes))
 ///l.Info("FaucetLayer2", "txHash sha256:", hex.EncodeToString(transferBytes))
 signature, err := zkpk.Sign(transferBytes)
 signatureSub, err := zkpk.Sign(bs)
 transferParams.Signature.Signature = signature.HexString()
 // format tx
 jsonTransfer, err := json.Marshal(transferParams)
 submitter := &SubmitterSignature{}
 submitter.PubKey = zkpk.GetPublicKey() //req.Zksign.PubKey
 submitter.Signature = signatureSub.HexString()
 jsonSubmitter, err := json.Marshal(submitter)

 privateKey, err := crypto.HexToECDSA(ethPrivateKey)
 if err != nil {
  log.Fatal(err)
 }

 signMessageTmp := `Transfer {amount} DAI to: {to}
Fee: {fee} DAI
Nonce: {nonce}`
 signMessage := strings.ReplaceAll(strings.ReplaceAll(strings.ReplaceAll(
  strings.ReplaceAll(signMessageTmp, "{amount}", damount.Div(e18).StringFixed(1)),
  "{to}", ToAddress,
 ), "{fee}", dfee.Div(e18).String()), "{nonce}", strconv.Itoa(int(transferParams.Nonce)))
 //
 fmt.Println("signMessage", signMessage)
 signMsgBytes := []byte(signMessage)
 fmt.Println("signMessage", signMessage)
 //hash := crypto.Keccak256Hash([]byte(signMessage)).Bytes()
 msg := fmt.Sprintf("\x19Ethereum Signed Message:\n%d%s", len(signMsgBytes), signMsgBytes)
 fmt.Println("[]byte(msg)", hex.EncodeToString([]byte(signMsgBytes)))
 hash := crypto.Keccak256Hash([]byte(msg))
 ethsignature, err := crypto.Sign(hash.Bytes(), privateKey)
 fmt.Println("signMessage11", ethsignature)
 ethAuth := zkl.EthSignature{
  Type:      "EthereumSignature",
  Signature: "0x" + hex.EncodeToString(ethsignature),
 }

 jsonEthAuth, err := json.Marshal(&ethAuth)
 tx := RPCTransaction{
  Id:      1,
  JSONRpc: "2.0",
  Method:  "sendTransaction",
  Params: []json.RawMessage{
   jsonTransfer,
   jsonEthAuth,
   jsonSubmitter,
  },
 }

 JsonTx, err := json.Marshal(tx)
 fmt.Println("JsonTx", string(JsonTx))
 res, err := post.PostJson("https://aws-gw-v2.zk.link/", JsonTx)
 fmt.Println("JsonTx", res, err)
 rsp := DtoRspFaucetLayer2{
  State: "ok",
 }
 return &rsp, nil
}
