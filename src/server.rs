use crate::methods;
use std::thread;
use jsonrpc_http_server::jsonrpc_core::{IoHandler, Value, Params};
use jsonrpc_http_server::ServerBuilder;

pub struct Entry {
    addr: String,
}

impl Clone for Entry {
    fn clone(&self) -> Entry {
        Entry {
            addr: self.addr.clone(),
        }
    }
}

impl Entry {
    pub fn new(addr: &str) -> Self {
        Entry {
            addr: String::from(addr),
        }
    }

    pub fn serve_silent(self) {
        thread::spawn(move || self.serve());
    }

    pub fn serve(self) {
        let server = ServerBuilder::new(self.setup_methods())
            .threads(3)
            .start_http(&self.addr.parse().unwrap())
            .unwrap();

        server.wait();
    }

    fn setup_methods(&self) -> IoHandler {
        let mut io = IoHandler::default();

        io.add_method(methods::WEB3_CLIENT_VERSION, |_params: Params| async {
            Ok(Value::String("Mist/v0.9.3/darwin/go1.4.1".to_owned()))
        });

        io.add_method(methods::WEB3_SHA3, |_params: Params| async {
            Ok(Value::String("0x47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad".to_owned()))
        });

        io.add_method(methods::NET_VERSION, |_params: Params| async {
            Ok(Value::String("3".to_owned()))
        });

        io.add_method(methods::NET_PEER_COUNT, |_params: Params| async {
            Ok(Value::String("0x2".to_owned()))
        });

        io.add_method(methods::NET_LISTENING, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::ETH_PROTOCOL_VERSION, |_params: Params| async {
            Ok(Value::String("54".to_owned()))
        });

        io.add_method(methods::ETH_SYNCING, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("startingBlock".to_string(), Value::String("0x384".to_owned()));
            data.insert("currentBlock".to_string(), Value::String("0x386".to_owned()));
            data.insert("highestBlock".to_string(), Value::String("0x454".to_owned()));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_COINBASE, |_params: Params| async {
            Ok(Value::String("0x407d73d8a49eeb85d32cf465507dd71d507100c1".to_owned()))
        });

        io.add_method(methods::ETH_MINING, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::ETH_HASHRATE, |_params: Params| async {
            Ok(Value::String("0x38a".to_owned()))
        });

        io.add_method(methods::ETH_GAS_PRICE, |_params: Params| async {
            Ok(Value::String("0x1dfd14000".to_owned()))
        });

        io.add_method(methods::ETH_ACCOUNTS, |_params: Params| async {
            Ok(Value::Array(vec![Value::String("0x407d73d8a49eeb85d32cf465507dd71d507100c1".to_owned())]))
        });

        io.add_method(methods::ETH_BLOCK_NUMBER, |_params: Params| async {
            Ok(Value::String("0x4b7".to_owned()))
        });

        io.add_method(methods::ETH_GET_BALANCE, |_params: Params| async {
            Ok(Value::String("0x0234c8a3397aab58".to_owned()))
        });

        io.add_method(methods::ETH_GET_STORAGE_AT, |_params: Params| async {
            Ok(Value::String("0x000000000000000000000000000000000000000000000000000000000000162e".to_owned()))
        });

        io.add_method(methods::ETH_GET_TRANSACTION_COUNT, |_params: Params| async {
            Ok(Value::String("0x1".to_owned()))
        });

        io.add_method(methods::ETH_GET_BLOCK_TRANSACTION_COUNT_BY_HASH, |_params: Params| async {
            Ok(Value::String("0xb".to_owned()))
        });

        io.add_method(methods::ETH_GET_BLOCK_TRANSACTION_COUNT_BY_NUMBER, |_params: Params| async {
            Ok(Value::String("0xa".to_owned()))
        });

        io.add_method(methods::ETH_GET_UNCLE_COUNT_BY_BLOCK_HASH, |_params: Params| async {
            Ok(Value::String("0x1".to_owned()))
        });

        io.add_method(methods::ETH_GET_UNCLE_COUNT_BY_BLOCK_NUMBER, |_params: Params| async {
            Ok(Value::String("0x1".to_owned()))
        });

        io.add_method(methods::ETH_GET_CODE, |_params: Params| async {
            Ok(Value::String("0x600160008035811a818181146012578301005b601b6001356025565b8060005260206000f25b600060078202905091905056".to_owned()))
        });

        io.add_method(methods::ETH_SIGN, |_params: Params| async {
            Ok(Value::String("0xa3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a12d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee1b".to_owned()))
        });

        io.add_method(methods::ETH_SIGN_TRANSACTION, |_params: Params| async {
            Ok(Value::String("0xa3f20717a250c2b0b729b7e5becbff67fdaef7e0699da4de7ca5895b02a170a12d887fd3b17bfdce3481f10bea41f45ba9f709d39ce8325427b57afcfc994cee1b".to_owned()))
        });

        io.add_method(methods::ETH_SEND_TRANSACTION, |_params: Params| async {
            Ok(Value::String("0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331".to_owned()))
        });

        io.add_method(methods::ETH_SEND_RAW_TRANSACTION, |_params: Params| async {
            Ok(Value::String("0xe670ec64341771606e55d6b4ca35a1a6b75ee3d5145a99d05921026d1527331".to_owned()))
        });

        io.add_method(methods::ETH_CALL, |_params: Params| async {
            Ok(Value::String("0x".to_owned()))
        });

        io.add_method(methods::ETH_ESTIMATE_GAS, |_params: Params| async {
            Ok(Value::String("0x5208".to_owned()))
        });

        io.add_method(methods::ETH_GET_BLOCK_BY_HASH, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("difficulty".to_string(), Value::String("0x4ea3f27bc".to_owned()));
            data.insert("extraData".to_string(), Value::String("0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32".to_owned()));
            data.insert("gasLimit".to_string(), Value::String("0x1388".to_owned()));
            data.insert("gasUsed".to_string(), Value::String("0x0".to_owned()));
            data.insert("hash".to_string(), Value::String("0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae".to_owned()));
            data.insert("logsBloom".to_string(), Value::String("0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_owned()));
            data.insert("miner".to_string(), Value::String("0xbb7b8287f3f0a933474a79eae42cbca977791171".to_owned()));
            data.insert("mixHash".to_string(), Value::String("0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843".to_owned()));
            data.insert("nonce".to_string(), Value::String("0x689056015818adbe".to_owned()));
            data.insert("number".to_string(), Value::String("0x1b4".to_owned()));
            data.insert("parentHash".to_string(), Value::String("0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54".to_owned()));
            data.insert("receiptsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("sha3Uncles".to_string(), Value::String("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_owned()));
            data.insert("size".to_string(), Value::String("0x220".to_owned()));
            data.insert("stateRoot".to_string(), Value::String("0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d".to_owned()));
            data.insert("timestamp".to_string(), Value::String("0x55ba467c".to_owned()));
            data.insert("totalDifficulty".to_string(), Value::String("0x78ed983323d".to_owned()));
            data.insert("transactions".to_string(), Value::Array(vec![]));
            data.insert("transactionsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("uncles".to_string(), Value::Array(vec![]));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_BLOCK_BY_NUMBER, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("difficulty".to_string(), Value::String("0x4ea3f27bc".to_owned()));
            data.insert("extraData".to_string(), Value::String("0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32".to_owned()));
            data.insert("gasLimit".to_string(), Value::String("0x1388".to_owned()));
            data.insert("gasUsed".to_string(), Value::String("0x0".to_owned()));
            data.insert("hash".to_string(), Value::String("0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae".to_owned()));
            data.insert("logsBloom".to_string(), Value::String("0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_owned()));
            data.insert("miner".to_string(), Value::String("0xbb7b8287f3f0a933474a79eae42cbca977791171".to_owned()));
            data.insert("mixHash".to_string(), Value::String("0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843".to_owned()));
            data.insert("nonce".to_string(), Value::String("0x689056015818adbe".to_owned()));
            data.insert("number".to_string(), Value::String("0x1b4".to_owned()));
            data.insert("parentHash".to_string(), Value::String("0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54".to_owned()));
            data.insert("receiptsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("sha3Uncles".to_string(), Value::String("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_owned()));
            data.insert("size".to_string(), Value::String("0x220".to_owned()));
            data.insert("stateRoot".to_string(), Value::String("0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d".to_owned()));
            data.insert("timestamp".to_string(), Value::String("0x55ba467c".to_owned()));
            data.insert("totalDifficulty".to_string(), Value::String("0x78ed983323d".to_owned()));
            data.insert("transactions".to_string(), Value::Array(vec![]));
            data.insert("transactionsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("uncles".to_string(), Value::Array(vec![]));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_UNCLE_BY_BLOCK_HASH_AND_INDEX, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("difficulty".to_string(), Value::String("0x4ea3f27bc".to_owned()));
            data.insert("extraData".to_string(), Value::String("0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32".to_owned()));
            data.insert("gasLimit".to_string(), Value::String("0x1388".to_owned()));
            data.insert("gasUsed".to_string(), Value::String("0x0".to_owned()));
            data.insert("hash".to_string(), Value::String("0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae".to_owned()));
            data.insert("logsBloom".to_string(), Value::String("0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_owned()));
            data.insert("miner".to_string(), Value::String("0xbb7b8287f3f0a933474a79eae42cbca977791171".to_owned()));
            data.insert("mixHash".to_string(), Value::String("0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843".to_owned()));
            data.insert("nonce".to_string(), Value::String("0x689056015818adbe".to_owned()));
            data.insert("number".to_string(), Value::String("0x1b4".to_owned()));
            data.insert("parentHash".to_string(), Value::String("0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54".to_owned()));
            data.insert("receiptsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("sha3Uncles".to_string(), Value::String("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_owned()));
            data.insert("size".to_string(), Value::String("0x220".to_owned()));
            data.insert("stateRoot".to_string(), Value::String("0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d".to_owned()));
            data.insert("timestamp".to_string(), Value::String("0x55ba467c".to_owned()));
            data.insert("totalDifficulty".to_string(), Value::String("0x78ed983323d".to_owned()));
            data.insert("transactions".to_string(), Value::Array(vec![]));
            data.insert("transactionsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("uncles".to_string(), Value::Array(vec![]));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_UNCLE_BY_BLOCK_NUMBER_AND_INDEX, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("difficulty".to_string(), Value::String("0x4ea3f27bc".to_owned()));
            data.insert("extraData".to_string(), Value::String("0x476574682f4c5649562f76312e302e302f6c696e75782f676f312e342e32".to_owned()));
            data.insert("gasLimit".to_string(), Value::String("0x1388".to_owned()));
            data.insert("gasUsed".to_string(), Value::String("0x0".to_owned()));
            data.insert("hash".to_string(), Value::String("0xdc0818cf78f21a8e70579cb46a43643f78291264dda342ae31049421c82d21ae".to_owned()));
            data.insert("logsBloom".to_string(), Value::String("0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_owned()));
            data.insert("miner".to_string(), Value::String("0xbb7b8287f3f0a933474a79eae42cbca977791171".to_owned()));
            data.insert("mixHash".to_string(), Value::String("0x4fffe9ae21f1c9e15207b1f472d5bbdd68c9595d461666602f2be20daf5e7843".to_owned()));
            data.insert("nonce".to_string(), Value::String("0x689056015818adbe".to_owned()));
            data.insert("number".to_string(), Value::String("0x1b4".to_owned()));
            data.insert("parentHash".to_string(), Value::String("0xe99e022112df268087ea7eafaf4790497fd21dbeeb6bd7a1721df161a6657a54".to_owned()));
            data.insert("receiptsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("sha3Uncles".to_string(), Value::String("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347".to_owned()));
            data.insert("size".to_string(), Value::String("0x220".to_owned()));
            data.insert("stateRoot".to_string(), Value::String("0xddc8b0234c2e0cad087c8b389aa7ef01f7d79b2570bccb77ce48648aa61c904d".to_owned()));
            data.insert("timestamp".to_string(), Value::String("0x55ba467c".to_owned()));
            data.insert("totalDifficulty".to_string(), Value::String("0x78ed983323d".to_owned()));
            data.insert("transactions".to_string(), Value::Array(vec![]));
            data.insert("transactionsRoot".to_string(), Value::String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421".to_owned()));
            data.insert("uncles".to_string(), Value::Array(vec![]));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_TRANSACTION_BY_HASH, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("blockHash".to_string(), Value::String("0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2".to_owned()));
            data.insert("blockNumber".to_string(), Value::String("0x5daf3b".to_owned()));
            data.insert("from".to_string(), Value::String("0xa7d9ddbe1f17865597fbd27ec712455208b6b76d".to_owned()));
            data.insert("gas".to_string(), Value::String("0xc350".to_owned()));
            data.insert("gasPrice".to_string(), Value::String("0x4a817c800".to_owned()));
            data.insert("hash".to_string(), Value::String("0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b".to_owned()));
            data.insert("input".to_string(), Value::String("0x68656c6c6f21".to_owned()));
            data.insert("nonce".to_string(), Value::String("0x15".to_owned()));
            data.insert("to".to_string(), Value::String("0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb".to_owned()));
            data.insert("transactionIndex".to_string(), Value::String("0x41".to_owned()));
            data.insert("value".to_string(), Value::String("0xf3dbb76162000".to_owned()));
            data.insert("v".to_string(), Value::String("0x25".to_owned()));
            data.insert("r".to_string(), Value::String("0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea".to_owned()));
            data.insert("s".to_string(), Value::String("0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c".to_owned()));
            data.insert("value".to_string(), Value::String("0xf3dbb76162000".to_owned()));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_TRANSACTION_BY_BLOCK_HASH_AND_INDEX, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("blockHash".to_string(), Value::String("0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2".to_owned()));
            data.insert("blockNumber".to_string(), Value::String("0x5daf3b".to_owned()));
            data.insert("from".to_string(), Value::String("0xa7d9ddbe1f17865597fbd27ec712455208b6b76d".to_owned()));
            data.insert("gas".to_string(), Value::String("0xc350".to_owned()));
            data.insert("gasPrice".to_string(), Value::String("0x4a817c800".to_owned()));
            data.insert("hash".to_string(), Value::String("0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b".to_owned()));
            data.insert("input".to_string(), Value::String("0x68656c6c6f21".to_owned()));
            data.insert("nonce".to_string(), Value::String("0x15".to_owned()));
            data.insert("to".to_string(), Value::String("0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb".to_owned()));
            data.insert("transactionIndex".to_string(), Value::String("0x41".to_owned()));
            data.insert("value".to_string(), Value::String("0xf3dbb76162000".to_owned()));
            data.insert("v".to_string(), Value::String("0x25".to_owned()));
            data.insert("r".to_string(), Value::String("0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea".to_owned()));
            data.insert("s".to_string(), Value::String("0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c".to_owned()));
            data.insert("value".to_string(), Value::String("0xf3dbb76162000".to_owned()));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_TRANSACTION_BY_BLOCK_NUMBER_AND_INDEX, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("blockHash".to_string(), Value::String("0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2".to_owned()));
            data.insert("blockNumber".to_string(), Value::String("0x5daf3b".to_owned()));
            data.insert("from".to_string(), Value::String("0xa7d9ddbe1f17865597fbd27ec712455208b6b76d".to_owned()));
            data.insert("gas".to_string(), Value::String("0xc350".to_owned()));
            data.insert("gasPrice".to_string(), Value::String("0x4a817c800".to_owned()));
            data.insert("hash".to_string(), Value::String("0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b".to_owned()));
            data.insert("input".to_string(), Value::String("0x68656c6c6f21".to_owned()));
            data.insert("nonce".to_string(), Value::String("0x15".to_owned()));
            data.insert("to".to_string(), Value::String("0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb".to_owned()));
            data.insert("transactionIndex".to_string(), Value::String("0x41".to_owned()));
            data.insert("value".to_string(), Value::String("0xf3dbb76162000".to_owned()));
            data.insert("v".to_string(), Value::String("0x25".to_owned()));
            data.insert("r".to_string(), Value::String("0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea".to_owned()));
            data.insert("s".to_string(), Value::String("0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c".to_owned()));
            data.insert("value".to_string(), Value::String("0xf3dbb76162000".to_owned()));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_TRANSACTION_RECEIPT, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("transactionHash".to_string(), Value::String("0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238".to_owned()));
            data.insert("transactionIndex".to_string(), Value::String("0x1".to_owned()));
            data.insert("blockNumber".to_string(), Value::String("0xb".to_owned()));
            data.insert("blockHash".to_string(), Value::String("0xc6ef2fc5426d6ad6fd9e2a26abeab0aa2411b7ab17f30a99d3cb96aed1d1055b".to_owned()));
            data.insert("cumulativeGasUsed".to_string(), Value::String("0x33bc".to_owned()));
            data.insert("gasUsed".to_string(), Value::String("0x4dc".to_owned()));
            data.insert("contractAddress".to_string(), Value::String("0xb60e8dd61c5d32be8058bb8eb970870f07233155".to_owned()));
            data.insert("logs".to_string(), Value::Array(vec![Value::String("...".to_owned())]));
            data.insert("logsBloom".to_string(), Value::String("0x00...0".to_owned()));
            data.insert("status".to_string(), Value::String("0x1".to_owned()));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_GET_COMPILERS, |_params: Params| async {
            Ok(Value::Array(vec![Value::String("solidity".to_owned()), Value::String("lll".to_owned()), Value::String("serpent".to_owned())]))
        });

        io.add_method(methods::ETH_COMPILE_LLL, |_params: Params| async {
            Ok(Value::String("0x603880600c6000396000f3006001600060e060020a600035048063c6888fa114601857005b6021600435602b565b8060005260206000f35b600081600702905091905056".to_owned()))
        });

        io.add_method(methods::ETH_COMPILE_SOLIDITY, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("code".to_string(), Value::String("0x605880600c6000396000f3006000357c010000000000000000000000000000000000000000000000000000000090048063c6888fa114602e57005b603d6004803590602001506047565b8060005260206000f35b60006007820290506053565b91905056".to_owned()));

            let mut info = jsonrpc_core::serde_json::Map::new();
            info.insert("source".to_string(), Value::String("contract test {\n   function multiply(uint a) constant returns(uint d) {\n       return a * 7;\n   }\n}\n".to_owned()));
            info.insert("language".to_string(), Value::String("Solidity".to_owned()));
            info.insert("languageVersion".to_string(), Value::String("0".to_owned()));
            info.insert("compilerVersion".to_string(), Value::String("0.9.19".to_owned()));

            let mut abi_definition = jsonrpc_core::serde_json::Map::new();
            abi_definition.insert("constant".to_string(), Value::Bool(true));
            abi_definition.insert("name".to_string(), Value::String("multiply".to_owned()));
            abi_definition.insert("type".to_string(), Value::String("function".to_owned()));

            let mut inputs = jsonrpc_core::serde_json::Map::new();
            inputs.insert("name".to_string(), Value::String("a".to_owned()));
            inputs.insert("type".to_string(), Value::String("uint256".to_owned()));

            let mut outputs = jsonrpc_core::serde_json::Map::new();
            outputs.insert("name".to_string(), Value::String("d".to_owned()));
            outputs.insert("type".to_string(), Value::String("uint256".to_owned()));

            abi_definition.insert("inputs".to_string(), Value::Array(vec![Value::Object(inputs)]));
            abi_definition.insert("outputs".to_string(), Value::Array(vec![Value::Object(outputs)]));
            info.insert("abiDefinition".to_string(), Value::Array(vec![Value::Object(abi_definition)]));
            data.insert("info".to_string(), Value::Object(info));

            Ok(Value::Object(data))
        });

        io.add_method(methods::ETH_COMPILE_SERPENT, |_params: Params| async {
            Ok(Value::String("0x603880600c6000396000f3006001600060e060020a600035048063c6888fa114601857005b6021600435602b565b8060005260206000f35b600081600702905091905056".to_owned()))
        });

        io.add_method(methods::ETH_NEW_FILTER, |_params: Params| async {
            Ok(Value::String("0x1".to_owned()))
        });

        io.add_method(methods::ETH_NEW_BLOCK_FILTER, |_params: Params| async {
            Ok(Value::String("0x1".to_owned()))
        });

        io.add_method(methods::ETH_NEW_PENDING_TRANSACTION_FILTER, |_params: Params| async {
            Ok(Value::String("0x1".to_owned()))
        });

        io.add_method(methods::ETH_UNINSTALL_FILTER, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::ETH_GET_FILTER_CHANGES, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("logIndex".to_string(), Value::String("0x1".to_owned()));
            data.insert("blockNumber".to_string(), Value::String("0x1b4".to_owned()));
            data.insert("blockHash".to_string(), Value::String("0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d".to_owned()));
            data.insert("transactionHash".to_string(), Value::String("0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf".to_owned()));
            data.insert("transactionIndex".to_string(), Value::String("0x0".to_owned()));
            data.insert("address".to_string(), Value::String("0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d".to_owned()));
            data.insert("data".to_string(), Value::String("0x0000000000000000000000000000000000000000000000000000000000000000".to_owned()));
            data.insert("topics".to_string(), Value::Array(vec![Value::String("0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5".to_owned())]));

            Ok(Value::Array(vec![Value::Object(data)]))
        });

        io.add_method(methods::ETH_GET_FILTER_LOGS, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("logIndex".to_string(), Value::String("0x1".to_owned()));
            data.insert("blockNumber".to_string(), Value::String("0x1b4".to_owned()));
            data.insert("blockHash".to_string(), Value::String("0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d".to_owned()));
            data.insert("transactionHash".to_string(), Value::String("0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf".to_owned()));
            data.insert("transactionIndex".to_string(), Value::String("0x0".to_owned()));
            data.insert("address".to_string(), Value::String("0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d".to_owned()));
            data.insert("data".to_string(), Value::String("0x0000000000000000000000000000000000000000000000000000000000000000".to_owned()));
            data.insert("topics".to_string(), Value::Array(vec![Value::String("0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5".to_owned())]));

            Ok(Value::Array(vec![Value::Object(data)]))
        });

        io.add_method(methods::ETH_GET_FILTER_LOGS, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("logIndex".to_string(), Value::String("0x1".to_owned()));
            data.insert("blockNumber".to_string(), Value::String("0x1b4".to_owned()));
            data.insert("blockHash".to_string(), Value::String("0x8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcfdf829c5a142f1fccd7d".to_owned()));
            data.insert("transactionHash".to_string(), Value::String("0xdf829c5a142f1fccd7d8216c5785ac562ff41e2dcfdf5785ac562ff41e2dcf".to_owned()));
            data.insert("transactionIndex".to_string(), Value::String("0x0".to_owned()));
            data.insert("address".to_string(), Value::String("0x16c5785ac562ff41e2dcfdf829c5a142f1fccd7d".to_owned()));
            data.insert("data".to_string(), Value::String("0x0000000000000000000000000000000000000000000000000000000000000000".to_owned()));
            data.insert("topics".to_string(), Value::Array(vec![Value::String("0x59ebeb90bc63057b6515673c3ecf9438e5058bca0f92585014eced636878c9a5".to_owned())]));

            Ok(Value::Array(vec![Value::Object(data)]))
        });

        io.add_method(methods::ETH_GET_WORK, |_params: Params| async {
            Ok(Value::Array(vec![Value::String("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_owned()), Value::String("0x5EED00000000000000000000000000005EED0000000000000000000000000000".to_owned()), Value::String("0xd1ff1c01710000000000000000000000d1ff1c01710000000000000000000000".to_owned())]))
        });

        io.add_method(methods::ETH_SUBMIT_WORK, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::ETH_SUBMIT_HASHRATE, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::DB_PUT_STRING, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::DB_GET_STRING, |_params: Params| async {
            Ok(Value::String("myString".to_owned()))
        });

        io.add_method(methods::DB_PUT_HEX, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::DB_GET_HEX, |_params: Params| async {
            Ok(Value::String("0x68656c6c6f20776f726c64".to_owned()))
        });

        io.add_method(methods::SHH_POST, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::DB_GET_HEX, |_params: Params| async {
            Ok(Value::String("2".to_owned()))
        });

        io.add_method(methods::SHH_NEW_IDENTITY, |_params: Params| async {
            Ok(Value::String("0xc931d93e97ab07fe42d923478ba2465f283f440fd6cabea4dd7a2c807108f651b7135d1d6ca9007d5b68aa497e4619ac10aa3b27726e1863c1fd9b570d99bbaf".to_owned()))
        });

        io.add_method(methods::SHH_HAS_IDENTITY, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::SHH_NEW_GROUP, |_params: Params| async {
            Ok(Value::String("0xc65f283f440fd6cabea4dd7a2c807108f651b7135d1d6ca90931d93e97ab07fe42d923478ba2407d5b68aa497e4619ac10aa3b27726e1863c1fd9b570d99bbaf".to_owned()))
        });

        io.add_method(methods::SHH_ADD_TO_GROUP, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::SHH_NEW_FILTER, |_params: Params| async {
            Ok(Value::String("0x7".to_owned()))
        });

        io.add_method(methods::SHH_UNINSTALL_FILTER, |_params: Params| async {
            Ok(Value::Bool(true))
        });

        io.add_method(methods::SHH_GET_FILTER_CHANGES, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("hash".to_string(), Value::String("0x33eb2da77bf3527e28f8bf493650b1879b08c4f2a362beae4ba2f71bafcd91f9".to_owned()));
            data.insert("from".to_string(), Value::String("0x3ec052fc33..".to_owned()));
            data.insert("to".to_string(), Value::String("0x87gdf76g8d7fgdfg...".to_owned()));
            data.insert("expiry".to_string(), Value::String("0x54caa50a".to_owned()));
            data.insert("sent".to_string(), Value::String("0x54ca9ea2".to_owned()));
            data.insert("ttl".to_string(), Value::String("0x64".to_owned()));
            data.insert("topics".to_string(), Value::Array(vec![Value::String("0x6578616d".to_owned())]));
            data.insert("payload".to_string(), Value::String("0x7b2274797065223a226d657373616765222c2263686...".to_owned()));
            data.insert("workProved".to_string(), Value::String("0x0".to_owned()));

            Ok(Value::Array(vec![Value::Object(data)]))
        });

        io.add_method(methods::SHH_GET_MESSAGES, |_params: Params| async {
            let mut data = jsonrpc_core::serde_json::Map::new();
            data.insert("hash".to_string(), Value::String("0x33eb2da77bf3527e28f8bf493650b1879b08c4f2a362beae4ba2f71bafcd91f9".to_owned()));
            data.insert("from".to_string(), Value::String("0x3ec052fc33..".to_owned()));
            data.insert("to".to_string(), Value::String("0x87gdf76g8d7fgdfg...".to_owned()));
            data.insert("expiry".to_string(), Value::String("0x54caa50a".to_owned()));
            data.insert("sent".to_string(), Value::String("0x54ca9ea2".to_owned()));
            data.insert("ttl".to_string(), Value::String("0x64".to_owned()));
            data.insert("topics".to_string(), Value::Array(vec![Value::String("0x6578616d".to_owned())]));
            data.insert("payload".to_string(), Value::String("0x7b2274797065223a226d657373616765222c2263686...".to_owned()));
            data.insert("workProved".to_string(), Value::String("0x0".to_owned()));

            Ok(Value::Array(vec![Value::Object(data)]))
        });

        io
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_web3_client_version() {
        Entry::new("127.0.0.1:8545").serve_silent();

        let mut map = HashMap::new();
        map.insert("jsonrpc", "2.0");
        map.insert("method", "web3_clientVersion");
        map.insert("params", "0x7");
        map.insert("id", "73");

        let client = reqwest::blocking::Client::new();

        let res = client.post("http://127.0.0.1:8545")
            .json(&map)
            .send();

        match res {
            Ok(data) => {
                match data.text() {
                    Ok(t) => {
                        assert_eq!(String::from(r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":"73"}"#.to_owned()+"\n"), t);
                    },
                    Err(err) => println!("{}", err),
                }
            },
            Err(err) => println!("{}", err),
        }
    }
}