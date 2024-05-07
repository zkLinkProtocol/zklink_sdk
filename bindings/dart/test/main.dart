import 'package:test/test.dart';
import 'dart:convert';
import '../lib/api.dart';
import '../lib/frb_generated.dart';

Future<void> main() async {
    await RustLib.init();
    test('Order Matching', () {
        var signer = Signer.ethSigner(ethPrivateKey: "0xbe725250b123a39dab5b7579334d5888987c72a58f4508062545fe6e08ca94f4");
        var maker = signer.createSignedOrder(order: Order(
            accountId: 5,
            subAccountId: 1,
            slotId: 1,
            nonce: 1,
            baseTokenId: 18,
            quoteTokenId: 17,
            amount: "10000000000000",
            price: "10000000000",
            isSell: true,
            makerFeeRate: 5,
            takerFeeRate: 3,
            hasSubsidy: false,
        ));
        var taker = signer.createSignedOrder(order: Order(
            accountId: 6,
            subAccountId: 1,
            slotId: 1,
            nonce: 1,
            baseTokenId: 18,
            quoteTokenId: 17,
            amount: "10000000000000",
            price: "10000000000",
            isSell: false,
            makerFeeRate: 5,
            takerFeeRate: 3,
            hasSubsidy: false,
        ));
        var contractPrices = [
            ContractPrice(pairId: 1, marketPrice: "100"),
            ContractPrice(pairId: 2, marketPrice: "200"),
            ContractPrice(pairId: 3, marketPrice: "300"),
            ContractPrice(pairId: 4, marketPrice: "400"),
        ];
        var marginPrices = [
            SpotPriceInfo(tokenId: 11, price: "100"),
            SpotPriceInfo(tokenId: 12, price: "200"),
            SpotPriceInfo(tokenId: 13, price: "300"),
        ];
        var tx = OrderMatching(
            accountId: 10,
            subAccountId: 1,
            taker: taker,
            maker: maker,
            fee: "1000000000",
            feeToken: 18,
            contractPrices: contractPrices,
            marginPrices: marginPrices,
            expectBaseAmount: "10000000000000000",
            expectQuoteAmount: "10000000000000000",
        );
        var txStr = signer.signOrderMatching(tx: tx);
        print(txStr);
        var txJson = jsonDecode(txStr);
        expect(txJson["signature"]["pubKey"], "0x7b173e25e484eed3461091430f81b2a5bd7ae792f69701dcb073cb903f812510");
        expect(txJson["signature"]["signature"], "a4d48b291495bdb2505dc14c4f04931f854662c9333d7bae13bc953e852ad428b132148c47418b92c216cb555a5a5e8caaba200d204b9ef7f2fefe52e848a003");
    });
}