/*
Creación de nuevo token: 
resim new-token-fixed 1000000 --description "The American dollar" --name "dollar" --symbol "USD"
------------------------------------------------------------------------------------------------
Traspaso de tokens de una cuenta a otra con resim: 
resim transfer 500,$usd $acct2
------------------------------------------------------------------------------------------------
Para cambiar de cuenta por defecto: 
resim set-default-account [account2_address] [account2_pubkey]
------------------------------------------------------------------------------------------------
Actualizar componente:
resim publish . --address $pack
*/
use scrypto::prelude::*;

blueprint! {
    struct DexP2p {
        token_compra: Vault,
        token_venta: Vault,
        registro_compra: HashMap<Address, (Decimal, Decimal)>,
        registro_venta: HashMap<Address, (Decimal, Decimal)>,
        comision: Decimal,
        caja_comision: Vault
        }

    impl DexP2p {
        pub fn new(deficion_compra: Address, deficion_venta: Address, fee: Decimal) -> Component {
            
            Self {
                token_compra: Vault::new(ResourceDef::from(deficion_compra)),
                token_venta: Vault::new(ResourceDef::from(deficion_venta)),
                registro_compra: HashMap::new(),
                registro_venta: HashMap::new(),
                comision: fee,
                caja_comision: Vault::new(RADIX_TOKEN)
            }
            .instantiate()
        }

        
        pub fn orden_compra(&mut self, direccion: Address, cantidad: Bucket, precio: Decimal, fee_xrd: Bucket) -> (Bucket,Bucket) {
            // Cobrar fee
            assert!(fee_xrd.amount() >= self.comision , "Comisión insuficiente");  
            self.caja_comision.put(fee_xrd.take(self.comision));

            // Insertamos registros de orden de compra
            self.registro_compra.insert(direccion, (cantidad.amount(), precio));

            // Trasladamos los token para la compra a el contendor del componente
            self.token_compra.put(cantidad.take(cantidad.amount()));

            // Devolvemos los Bucket usados
            (cantidad, fee_xrd)
        }
      
        pub fn ver_orden_compra(&self) {
            // Listar las ordenes de compra 
            for (address, ordenes) in &self.registro_compra {
                info!("Direccion: {}: {:?}", address, ordenes);
            };
        }

        pub fn ejecutar_orden_compra(&mut self, direccion: Address, pago: Bucket, fee_xrd: Bucket) -> (Bucket, Bucket, Bucket) {
            // Cobrar fee
            assert!(fee_xrd.amount() >= self.comision , "Comisión insuficiente");  
            self.caja_comision.put(fee_xrd.take(self.comision));

            // Seleccionamos orden compra
            let orden = self.registro_compra.get(&direccion).unwrap();

            // Calculamos la cantidad de token necesarios para realizar la compra
            let cantidad: Decimal = orden.0 / orden.1;

            // comprobación saldo suficiente
            assert!(pago.amount() >= cantidad, "Saldo insuficiente");  

            // ejecutar compra:
            //    1. pasar los xrd del comprador 
            let cuenta_vendedor =  Account::from(direccion);
            cuenta_vendedor.deposit(pago.take(cantidad));
            //    2. pasar los token compra al vendedor
            let deposito: Bucket = self.token_compra.take(orden.0);
            //    3. eliminar la orden de compra 
            self.registro_compra.remove(&direccion);

            // Devolvemos los Bucket usados
            (pago, deposito, fee_xrd)
        }
    }
}
