# 🎨 TrazArte - Registro de Autenticidad para Artesanías en Solana

TrazArte es un contrato inteligente (Smart Contract) desarrollado en **Rust** utilizando el framework **Anchor**. Este proyecto fue creado como parte del Bootcamp de certificación de la **Solana Foundation**.

El objetivo principal de TrazArte es proporcionar un registro inmutable y seguro en la blockchain para obras de arte manuales (cerámica, textiles, tallados, etc.). Permite a los creadores registrar sus piezas, certificar su autenticidad, actualizar su estado (ej. "En taller", "Vendida") y prevenir la falsificación comercial.

---

## ⚙️ Arquitectura Técnica

El programa implementa un patrón **CRUD completo (Create, Read, Update, Delete)** respaldado por **PDAs (Program Derived Addresses)** para el manejo eficiente y seguro del estado.

### Estructura de Datos (Estado)
Cada pieza artesanal se almacena en su propia cuenta (PDA) con la siguiente estructura:
- `artesano` (Pubkey): Billetera del creador original.
- `id_pieza` (u64): Identificador numérico único de la pieza.
- `material` (String): Material principal de la obra (ej. "Cerámica de barro").
- `region` (String): Lugar de origen (ej. "Zumpango de Ocampo").
- `estado` (String): Estado actual de la pieza ("En taller", "En exhibición", "Vendida").

### Seguridad y PDAs
La dirección de la cuenta de cada artesanía se deriva de manera determinista utilizando las siguientes semillas (seeds):
`[b"artesania", artesano.key().as_ref(), id_pieza.to_le_bytes().as_ref()]`

Esto asegura que:
1. Un artesano puede registrar múltiples piezas sin colisión de cuentas (gracias al `id_pieza`).
2. Se utiliza la macro `has_one = artesano` en las instrucciones de actualización y eliminación para garantizar que **solo el creador original** pueda modificar o destruir el registro.

---

## 🚀 Guía de Ejecución en Solana Playground (SolPG)

Este proyecto está diseñado para ser compilado, desplegado y probado completamente desde el navegador usando [Solana Playground (beta.solpg.io)](https://beta.solpg.io/).

### Paso 1: Configuración del Entorno
1. Abre [Solana Playground](https://beta.solpg.io/).
2. Crea un nuevo proyecto seleccionando el framework **Anchor** y nómbralo `trazarte`.
3. Copia y pega el código del contrato inteligente en el archivo `src/lib.rs`.
4. Copia y pega el código de pruebas TypeScript en el archivo `client/client.ts` (o en la carpeta tests).

### Paso 2: Construcción (Build)
1. Ve a la pestaña de herramientas en el menú lateral izquierdo (icono de martillo y llave inglesa).
2. Haz clic en el botón **Build** (o escribe `anchor build` en la terminal).
3. Asegúrate de que aparezca una marca de verificación verde indicando que compiló correctamente.
   - *Nota: SolPG actualizará automáticamente el `declare_id!` en tu código con el ID de programa generado para ti.*

### Paso 3: Configurar Billetera y Obtener SOL de Prueba
1. En la parte inferior izquierda de la pantalla, verifica que tu billetera esté conectada a la **Devnet**.
2. Abre la terminal de SolPG y solicita tokens de prueba (SOL) para pagar el despliegue ejecutando:
   
      ```bash
      solana airdrop 2
      ```
3. Verifica tu balance ejecutando:
   
      ```Bash
      solana balance
      ```
### Paso 4: Despliegue (Deploy)
1. Regresa a la pestaña de herramientas (Build & Deploy).
2. Haz clic en el botón Deploy (o escribe anchor deploy en la terminal).
3. Espera a que la terminal confirme el proceso con el mensaje: Deploy successful. ¡Tu contrato ya vive en la blockchain!

### Paso 5: Pruebas (Test CRUD)
Para probar que el contrato funciona correctamente, ejecutaremos el script de cliente que simula la interacción de una página web (dApp) con nuestro programa.

1. Escribe el siguiente comando en la terminal para ejecutar el archivo client.ts:
   
      ```Bash
        run
      ```
2. El script ejecutará automáticamente el ciclo de vida completo de la pieza:
  -CREATE: Generará la PDA y registrará la pieza.
  -READ: Extraerá los datos de la blockchain y los imprimirá en consola.
  -UPDATE: Cambiará el estado de la pieza a "Vendida y Exportada" y validará el cambio.
  -DELETE: Eliminará la cuenta de la pieza de la blockchain, demostrando la recuperación de los tokens SOL retenidos por la renta (rent).

### 📋 Salida Esperada en Consola (Test)
Al ejecutar el comando run, deberías ver una salida similar a esta en tu terminal:

```Bash
Iniciando pruebas del contrato TrazArte...

📍 PDA derivada para la pieza: [Dirección Pública de la PDA]

--- 1. CREANDO REGISTRO ---
✅ Transacción de creación exitosa. Hash: [Hash de Transacción]

--- 2. LEYENDO DATOS ---
📦 Datos extraídos de la PDA:
   - Artesano dueño: [Tu Billetera]
   - ID de Pieza: 1
   - Material: Cerámica de barro
   - Región de origen: Zumpango de Ocampo
   - Estado actual: En taller

--- 3. ACTUALIZANDO ESTADO ---
✅ Transacción de actualización exitosa. Hash: [Hash de Transacción]
🔄 Nuevo estado verificado en la blockchain: Vendida y Exportada

--- 4. ELIMINANDO REGISTRO ---
✅ Transacción de eliminación (burn) exitosa. Hash: [Hash de Transacción]
💰 La cuenta fue cerrada y los tokens SOL de 'rent' han vuelto a tu wallet.

🚀 ¡Prueba del CRUD completada con éxito!
```
## 🛠️ Tecnologías
- Rust - Lógica del Smart Contract.

- Anchor Framework - Simplificación del desarrollo y validación de cuentas.

- TypeScript & Web3.js - Integración de frontend y pruebas (Testing).

- Solana Devnet - Red de pruebas para despliegue.
