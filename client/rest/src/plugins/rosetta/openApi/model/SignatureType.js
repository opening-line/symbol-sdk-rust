/**
 * Rosetta
 * Build Once. Integrate Your Blockchain Everywhere.
 *
 * The version of the OpenAPI document: 1.4.13
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 *
 */

import ApiClient from '../ApiClient.js';
/**
* Enum class SignatureType.
* @enum {}
* @readonly
*/
export default class SignatureType {
    
        /**
         * value: "ecdsa"
         * @const
         */
        "ecdsa" = "ecdsa";

    
        /**
         * value: "ecdsa_recovery"
         * @const
         */
        "ecdsa_recovery" = "ecdsa_recovery";

    
        /**
         * value: "ed25519"
         * @const
         */
        "ed25519" = "ed25519";

    
        /**
         * value: "schnorr_1"
         * @const
         */
        "schnorr_1" = "schnorr_1";

    
        /**
         * value: "schnorr_bip340"
         * @const
         */
        "schnorr_bip340" = "schnorr_bip340";

    
        /**
         * value: "schnorr_poseidon"
         * @const
         */
        "schnorr_poseidon" = "schnorr_poseidon";

    

    /**
    * Returns a <code>SignatureType</code> enum value from a Javascript object name.
    * @param {Object} data The plain JavaScript object containing the name of the enum value.
    * @return {module:model/SignatureType} The enum <code>SignatureType</code> value.
    */
    static constructFromObject(object) {
        return object;
    }
}

