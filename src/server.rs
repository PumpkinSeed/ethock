use crate::methods;

pub struct PayloadWrapper {
    jsonrpc: String,
    method: String,
    params: Vec<String>,
    id: String,
}

pub struct Entry {
    addr: String,
}

impl Entry {
    pub fn new(addr: &str) -> Self {
        Entry {
            addr: String::from(addr),
        }
    }

    pub fn serve(self) {
        rouille::start_server(self.addr, move |request| {
            router!(request,
            (GET) (/) => {
                rouille::Response::text("hello world")
            },

            _ => rouille::Response::empty_404()
        )
        });
    }

    fn fake(payload: PayloadWrapper) -> Option<String> {
        match &payload.method[..] {
            methods::WEB3_CLIENT_VERSION => {
                Some(String::from(r#"{"id":67,"jsonrpc":"2.0","result":"Mist/v0.9.3/darwin/go1.4.1"}"#))
            },
            methods::WEB3_SHA3 => {
                Some(String::from(r#"{"id":64,"jsonrpc":"2.0","result":"0x47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad"}"#))
            },
            methods::NET_VERSION => {
                Some(String::from(r#"{"id":67,"jsonrpc":"2.0","result":"3"}"#))
            },
            methods::NET_PEER_COUNT => {
                Some(String::from(r#"{"id":74,"jsonrpc":"2.0","result":"0x2"}"#))
            },
            methods::NET_LISTENING => {
                Some(String::from(r#"{"id":67,"jsonrpc":"2.0","result":true}"#))
            },
            methods::ETH_PROTOCOL_VERSION => {
                Some(String::from(r#"{"id":67,"jsonrpc":"2.0","result":"54"}"#))
            },
            methods::ETH_SYNCING => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":{"startingBlock":"0x384","currentBlock":"0x386","highestBlock":"0x454"}}"#))
            },
            methods::ETH_COINBASE => {
                Some(String::from(r#"{"id":64,"jsonrpc":"2.0","result":"0x407d73d8a49eeb85d32cf465507dd71d507100c1"}"#))
            },
            methods::ETH_MINING => {
                Some(String::from(r#"{"id":71,"jsonrpc":"2.0","result":true}"#))
            },
            methods::ETH_HASHRATE => {
                Some(String::from(r#"{"id":71,"jsonrpc":"2.0","result":"0x38a"}"#))
            },
            methods::ETH_GAS_PRICE => {
                Some(String::from(r#"{"id":73,"jsonrpc":"2.0","result":"0x1dfd14000"}"#))
            },
            methods::ETH_ACCOUNTS => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":["0x407d73d8a49eeb85d32cf465507dd71d507100c1"]}"#))
            },
            methods::ETH_BLOCK_NUMBER => {
                Some(String::from(r#"{"id":83,"jsonrpc":"2.0","result":"0x4b7"}"#))
            },
            methods::ETH_GET_BALANCE => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x0234c8a3397aab58"}"#))
            },
            methods::ETH_GET_STORAGE_AT => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":"0x000000000000000000000000000000000000000000000000000000000000162e"}"#))
            },
            methods::ETH_GET_TRANSACTION_COUNT => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x1"}"#))
            },
            methods::ETH_GET_BLOCK_TRANSACTION_COUNT_BY_HASH => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xb"}"#))
            },
            methods::ETH_GET_BLOCK_TRANSACTION_COUNT_BY_NUMBER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xa"}"#))
            },
            methods::ETH_GET_UNCLE_COUNT_BY_BLOCK_HASH => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x1"}"#))
            },
            methods::ETH_GET_UNCLE_COUNT_BY_BLOCK_NUMBER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x1"}"#))
            },
            methods::ETH_GET_CODE => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x600160008035811a818181146012578301005b601b6001356025565b8060005260206000f25b600060078202905091905056"}"#))
            },
            methods::ETH_SIGN => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xa3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a12d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee1b"}"#))
            },
            methods::ETH_SIGN_TRANSACTION => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xa3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a12d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee1b"}"#))
            },
            methods::ETH_SEND_TRANSACTION => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331"}"#))
            },
            methods::ETH_SEND_RAW_TRANSACTION => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331"}"#))
            },
            methods::ETH_CALL => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x"}"#))
            },
            methods::ETH_ESTIMATE_GAS => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x5208"}"#))
            },
            methods::ETH_GET_BLOCK_BY_HASH => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":{"difficulty":"0x4ea3f27bc","extraData":"0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32","gasLimit":"0x1388","gasUsed":"0x0","hash":"0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","miner":"0xbb7b8287f3f0a933474a79eae42cbca977791171","mixHash":"0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843","nonce":"0x689056015818adbe","number":"0x1b4","parentHash":"0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54","receiptsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","size":"0x220","stateRoot":"0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d","timestamp":"0x55ba467c","totalDifficulty":"0x78ed983323d","transactions":[],"transactionsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","uncles":[]}}"#))
            },
            methods::ETH_GET_BLOCK_BY_NUMBER => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":{"difficulty":"0x4ea3f27bc","extraData":"0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32","gasLimit":"0x1388","gasUsed":"0x0","hash":"0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","miner":"0xbb7b8287f3f0a933474a79eae42cbca977791171","mixHash":"0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843","nonce":"0x689056015818adbe","number":"0x1b4","parentHash":"0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54","receiptsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","size":"0x220","stateRoot":"0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d","timestamp":"0x55ba467c","totalDifficulty":"0x78ed983323d","transactions":[],"transactionsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","uncles":[]}}"#))
            },
            methods::ETH_GET_TRANSACTION_BY_HASH => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":{"blockHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2","blockNumber":"0x5daf3b","from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d","gas":"0xc350","gasPrice":"0x4a817c800","hash":"0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b","input":"0x68656c6c6f21","nonce":"0x15","to":"0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb","transactionIndex":"0x41","value":"0xf3dbb76162000","v":"0x25","r":"0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea","s":"0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c"}}"#))
            },
            methods::ETH_GET_TRANSACTION_BY_BLOCK_HASH_AND_INDEX => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":{"blockHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2","blockNumber":"0x5daf3b","from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d","gas":"0xc350","gasPrice":"0x4a817c800","hash":"0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b","input":"0x68656c6c6f21","nonce":"0x15","to":"0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb","transactionIndex":"0x41","value":"0xf3dbb76162000","v":"0x25","r":"0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea","s":"0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c"}}"#))
            },
            methods::ETH_GET_TRANSACTION_BY_BLOCK_NUMBER_AND_INDEX => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":{"blockHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2","blockNumber":"0x5daf3b","from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d","gas":"0xc350","gasPrice":"0x4a817c800","hash":"0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b","input":"0x68656c6c6f21","nonce":"0x15","to":"0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb","transactionIndex":"0x41","value":"0xf3dbb76162000","v":"0x25","r":"0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea","s":"0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c"}}"#))
            },
            methods::ETH_GET_TRANSACTION_RECEIPT => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":{"transactionHash":"0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238","transactionIndex":"0x1","blockNumber":"0xb","blockHash":"0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b","cumulativeGasUsed":"0x33bc","gasUsed":"0x4dc","contractAddress":"0xb60e8dd61c5d32be8058bb8eb970870f07233155","logs":["..."],"logsBloom":"0x00...0","status":"0x1"}}"#))
            },
            methods::ETH_GET_UNCLE_BY_BLOCK_HASH_AND_INDEX => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":{"difficulty":"0x4ea3f27bc","extraData":"0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32","gasLimit":"0x1388","gasUsed":"0x0","hash":"0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","miner":"0xbb7b8287f3f0a933474a79eae42cbca977791171","mixHash":"0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843","nonce":"0x689056015818adbe","number":"0x1b4","parentHash":"0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54","receiptsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","size":"0x220","stateRoot":"0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d","timestamp":"0x55ba467c","totalDifficulty":"0x78ed983323d","transactions":[],"transactionsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","uncles":[]}}"#))
            },
            methods::ETH_GET_UNCLE_BY_BLOCK_NUMBER_AND_INDEX => {
                Some(String::from(r#"{"jsonrpc":"2.0","id":1,"result":{"difficulty":"0x4ea3f27bc","extraData":"0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32","gasLimit":"0x1388","gasUsed":"0x0","hash":"0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","miner":"0xbb7b8287f3f0a933474a79eae42cbca977791171","mixHash":"0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843","nonce":"0x689056015818adbe","number":"0x1b4","parentHash":"0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54","receiptsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","size":"0x220","stateRoot":"0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d","timestamp":"0x55ba467c","totalDifficulty":"0x78ed983323d","transactions":[],"transactionsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","uncles":[]}}"#))
            },
            methods::ETH_GET_COMPILERS => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":["solidity","lll","serpent"]}"#))
            },
            methods::ETH_COMPILE_LLL => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x603880600c6000396000f3006001600060e060020a600035048063c6888fa114601857005b6021600435602b565b8060005260206000f35b600081600702905091905056"}"#))
            },
            methods::ETH_COMPILE_SOLIDITY => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":{"code":"0x605880600c6000396000f3006000357c010000000000000000000000000000000000000000000000000000000090048063c6888fa114602e57005b603d6004803590602001506047565b8060005260206000f35b60006007820290506053565b91905056","info":{"source":"contract test {\n   function multiply(uint a) constant returns(uint d) {\n       return a * 7;\n   }\n}\n","language":"Solidity","languageVersion":"0","compilerVersion":"0.9.19","abiDefinition":[{"constant":true,"inputs":[{"name":"a","type":"uint256"}],"name":"multiply","outputs":[{"name":"d","type":"uint256"}],"type":"function"}],"userDoc":{"methods":{}},"developerDoc":{"methods":{}}}}}"#))
            },
            methods::ETH_COMPILE_SERPENT => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x603880600c6000396000f3006001600060e060020a600035048063c6888fa114601857005b6021600435602b565b8060005260206000f35b600081600702905091905056"}"#))
            },
            methods::ETH_NEW_FILTER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x1"}"#))
            },
            methods::ETH_NEW_BLOCK_FILTER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x1"}"#))
            },
            methods::ETH_NEW_PENDING_TRANSACTION_FILTER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x1"}"#))
            },
            methods::ETH_UNINSTALL_FILTER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":true}"#))
            },
            methods::ETH_GET_FILTER_CHANGES => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":[{"logIndex":"0x1","blockNumber":"0x1b4","blockHash":"0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d","transactionHash":"0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf","transactionIndex":"0x0","address":"0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d","data":"0x0000000000000000000000000000000000000000000000000000000000000000","topics":["0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5"]}]}"#))
            },
            methods::ETH_GET_FILTER_LOGS => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":[{"logIndex":"0x1","blockNumber":"0x1b4","blockHash":"0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d","transactionHash":"0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf","transactionIndex":"0x0","address":"0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d","data":"0x0000000000000000000000000000000000000000000000000000000000000000","topics":["0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5"]}]}"#))
            },
            methods::ETH_GET_LOGS => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":[{"logIndex":"0x1","blockNumber":"0x1b4","blockHash":"0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d","transactionHash":"0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf","transactionIndex":"0x0","address":"0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d","data":"0x0000000000000000000000000000000000000000000000000000000000000000","topics":["0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5"]}]}"#))
            },
            methods::ETH_GET_WORK => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":["0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef","0x5EED00000000000000000000000000005EED0000000000000000000000000000","0xd1ff1c01710000000000000000000000d1ff1c01710000000000000000000000"]}"#))
            },
            methods::ETH_SUBMIT_WORK => {
                Some(String::from(r#"{"id":73,"jsonrpc":"2.0","result":true}"#))
            },
            methods::ETH_SUBMIT_HASHRATE => {
                Some(String::from(r#"{"id":73,"jsonrpc":"2.0","result":true}"#))
            },
            methods::DB_PUT_STRING => {
                Some(String::from(r#"{"id":73,"jsonrpc":"2.0","result":true}"#))
            },
            methods::DB_GET_STRING => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"myString"}"#))
            },
            methods::DB_PUT_HEX => {
                Some(String::from(r#"{"id":73,"jsonrpc":"2.0","result":true}"#))
            },
            methods::DB_GET_HEX => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x68656c6c6f20776f726c64"}"#))
            },
            methods::SHH_POST => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":true}"#))
            },
            methods::SHH_VERSION => {
                Some(String::from(r#"{"id":67,"jsonrpc":"2.0","result":"2"}"#))
            },
            methods::SHH_NEW_IDENTITY => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xc931d93e97ab07fe42d923478ba2465f283f440fd6cabea4dd7a2c807108f651b7135d1d6ca9007d5b68aa497e4619ac10aa3b27726e1863c1fd9b570d99bbaf"}"#))
            },
            methods::SHH_HAS_IDENTITY => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":true}"#))
            },
            methods::SHH_NEW_GROUP => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0xc65f283f440fd6cabea4dd7a2c807108f651b7135d1d6ca90931d93e97ab07fe42d923478ba2407d5b68aa497e4619ac10aa3b27726e1863c1fd9b570d99bbaf"}"#))
            },
            methods::SHH_ADD_TO_GROUP => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":true}"#))
            },
            methods::SHH_NEW_FILTER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":"0x7"}"#))
            },
            methods::SHH_UNINSTALL_FILTER => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":true}"#))
            },
            methods::SHH_GET_FILTER_CHANGES => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":[{"hash":"0x33eb2da77bf3527e28f8bf493650b1879b08c4f2a362beae4ba2f71bafcd91f9","from":"0x3ec052fc33..","to":"0x87gdf76g8d7fgdfg...","expiry":"0x54caa50a","sent":"0x54ca9ea2","ttl":"0x64","topics":["0x6578616d"],"payload":"0x7b2274797065223a226d657373616765222c2263686...","workProved":"0x0"}]}"#))
            },
            methods::SHH_GET_MESSAGES => {
                Some(String::from(r#"{"id":1,"jsonrpc":"2.0","result":[{"hash":"0x33eb2da77bf3527e28f8bf493650b1879b08c4f2a362beae4ba2f71bafcd91f9","from":"0x3ec052fc33..","to":"0x87gdf76g8d7fgdfg...","expiry":"0x54caa50a","sent":"0x54ca9ea2","ttl":"0x64","topics":["0x6578616d"],"payload":"0x7b2274797065223a226d657373616765222c2263686...","workProved":"0x0"}]}"#))
            },
            _ => None,
        }
    }
}