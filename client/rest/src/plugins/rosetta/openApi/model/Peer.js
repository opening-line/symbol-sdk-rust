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
 * The Peer model module.
 * @module model/Peer
 * @version 1.4.13
 */
class Peer {
	/**
	 * Constructs a new <code>Peer</code>.
	 * A Peer is a representation of a node&#39;s peer.
	 * @alias module:model/Peer
	 * @param peerId {String}
	 */
	constructor(peerId) {

		Peer.initialize(this, peerId);
	}

	/**
	 * Initializes the fields of this object.
	 * This method is used by the constructors of any subclasses, in order to implement multiple inheritance (mix-ins).
	 * Only for internal use.
	 */
	static initialize(obj, peerId) {
		obj['peer_id'] = peerId;
	}

	/**
	 * Constructs a <code>Peer</code> from a plain JavaScript object, optionally creating a new instance.
	 * Copies all relevant properties from <code>data</code> to <code>obj</code> if supplied or a new instance if not.
	 * @param {Object} data The plain JavaScript object bearing properties of interest.
	 * @param {module:model/Peer} obj Optional instance to populate.
	 * @return {module:model/Peer} The populated <code>Peer</code> instance.
	 */
	static constructFromObject(data, obj) {
		if (data) {
			obj = obj || new Peer();

			if (data.hasOwnProperty('peer_id')) {
				obj['peer_id'] = ApiClient.convertToType(data['peer_id'], 'String');
			}
			if (data.hasOwnProperty('metadata')) {
				obj['metadata'] = ApiClient.convertToType(data['metadata'], Object);
			}
		}
		return obj;
	}

	/**
	 * Validates the JSON data with respect to <code>Peer</code>.
	 * @param {Object} data The plain JavaScript object bearing properties of interest.
	 * @return {boolean} to indicate whether the JSON data is valid with respect to <code>Peer</code>.
	 */
	static validateJSON(data) {
		// check to make sure all required properties are present in the JSON string
		for (const property of Peer.RequiredProperties) {
			if (!data.hasOwnProperty(property)) {
				throw new Error("The required field `" + property + "` is not found in the JSON data: " + JSON.stringify(data));
			}
		}
		// ensure the json data is a string
		if (data['peer_id'] && !(typeof data['peer_id'] === 'string' || data['peer_id'] instanceof String)) {
			throw new Error("Expected the field `peer_id` to be a primitive type in the JSON string but got " + data['peer_id']);
		}

		return true;
	}


}

Peer.RequiredProperties = ["peer_id"];

/**
 * @member {String} peer_id
 */
Peer.prototype['peer_id'] = undefined;

/**
 * @member {Object} metadata
 */
Peer.prototype['metadata'] = undefined;






export default Peer;

