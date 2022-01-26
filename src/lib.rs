/*
Creación de nuevo token: resim new-token-fixed 1000 --description "The American dollar" --name "dollar" --symbol "USD"
Traspaso de tokens de una cuenta a otra con resim: resim transfer 500,$usd $acct2
*/
use scrypto::prelude::*;

blueprint! {
    struct DexP2p {
        token_compra: Vault,
        token_venta: Vault,
        registro_compra: HashMap<u128, (Address, Decimal, Decimal)>,
        registro_venta: HashMap<u128, (Address, Decimal, Decimal)>
    }

    impl DexP2p {
        pub fn new(referencia: Address) -> Component {
            
            Self {
                token_compra: Vault::new(ResourceDef::from(referencia)),
                token_venta: Vault::new(RADIX_TOKEN),
                registro_compra: HashMap::new(),
                registro_venta: HashMap::new()
            }
            .instantiate()
        }

        pub fn orden_compra(&mut self, direccion: Address, cantidad: Bucket, precio: Decimal) -> Bucket {
            self.registro_compra.insert(Uuid::generate(), (direccion, cantidad.amount(), precio));
            self.token_compra.put(cantidad.take(cantidad.amount()));
            cantidad
        }
    }
}
