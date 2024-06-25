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
import PublicKey from './PublicKey.js';
import SignatureType from './SignatureType.js';
import SigningPayload from './SigningPayload.js';

/**
 * The Signature model module.
 * @module model/Signature
 * @version 1.4.13
 */
class Signature {
    /**
     * Constructs a new <code>Signature</code>.
     * Signature contains the payload that was signed, the public keys of the keypairs used to produce the signature, the signature (encoded in hex), and the SignatureType. PublicKey is often times not known during construction of the signing payloads but may be needed to combine signatures properly.
     * @alias module:model/Signature
     * @param signingPayload {module:model/SigningPayload} 
     * @param publicKey {module:model/PublicKey} 
     * @param signatureType {module:model/SignatureType} 
     * @param hexBytes {String} 
     */
    constructor(signingPayload, publicKey, signatureType, hexBytes) { 
        
        Signature.initialize(this, signingPayload, publicKey, signatureType, hexBytes);
    }

    /**
     * Initializes the fields of this object.
     * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
     * Only for internal use.
     */
    static initialize(obj, signingPayload, publicKey, signatureType, hexBytes) { 
        obj['signing_payload'] = signingPayload;
        obj['public_key'] = publicKey;
        obj['signature_type'] = signatureType;
        obj['hex_bytes'] = hexBytes;
    }

    /**
     * Constructs a <code>Signature</code> from a plain JavaScript object, optionally creating a new instance.
     * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @param {module:model/Signature} obj Optional instance to populate.
     * @return {module:model/Signature} The populated <code>Signature</code> instance.
     */
    static constructFromObject(data, obj) {
        if (data) {
            obj = obj || new Signature();

            if (data.hasOwnProperty('signing_payload')) {
                obj['signing_payload'] = SigningPayload.constructFromObject(data['signing_payload']);
            }
            if (data.hasOwnProperty('public_key')) {
                obj['public_key'] = PublicKey.constructFromObject(data['public_key']);
            }
            if (data.hasOwnProperty('signature_type')) {
                obj['signature_type'] = SignatureType.constructFromObject(data['signature_type']);
            }
            if (data.hasOwnProperty('hex_bytes')) {
                obj['hex_bytes'] = ApiClient.convertToType(data['hex_bytes'], 'String');
            }
        }
        return obj;
    }

    /**
     * Validates the JSON data with respect to <code>Signature</code>.
     * @param {Object} data The plain JavaScript object bearing properties of interest.
     * @return {boolean} to indicate whether the JSON data is valid with respect to <code>Signature</code>.
     */
    static validateJSON(data) {
        // check to make sure all required properties are present in the JSON string
        for (const property of Signature.RequiredProperties) {
            if (!data.hasOwnProperty(property)) {
                throw new Error("The required field `" + property + "` is not found in the JSON data: " + JSON.stringify(data));
            }
        }
        // validate the optional field `signing_payload`
        if (data['signing_payload']) { // data not null
          SigningPayload.validateJSON(data['signing_payload']);
        }
        // validate the optional field `public_key`
        if (data['public_key']) { // data not null
          PublicKey.validateJSON(data['public_key']);
        }
        // ensure the json data is a string
        if (data['hex_bytes'] && !(typeof data['hex_bytes'] === 'string' || data['hex_bytes'] instanceof String)) {
            throw new Error("Expected the field `hex_bytes` to be a primitive type in the JSON string but got " + data['hex_bytes']);
        }

        return true;
    }


}

Signature.RequiredProperties = ["signing_payload", "public_key", "signature_type", "hex_bytes"];

/**
 * @member {module:model/SigningPayload} signing_payload
 */
Signature.prototype['signing_payload'] = undefined;

/**
 * @member {module:model/PublicKey} public_key
 */
Signature.prototype['public_key'] = undefined;

/**
 * @member {module:model/SignatureType} signature_type
 */
Signature.prototype['signature_type'] = undefined;

/**
 * @member {String} hex_bytes
 */
Signature.prototype['hex_bytes'] = undefined;






export default Signature;

