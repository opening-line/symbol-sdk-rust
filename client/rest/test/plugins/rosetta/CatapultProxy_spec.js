/*
 * Copyright (c) 2016-2019, Jaguar0625, gimre, BloodyRookie, Tech Bureau, Corp.
 * Copyright (c) 2020-present, Jaguar0625, gimre, BloodyRookie.
 * All rights reserved.
 *
 * This file is part of Catapult.
 *
 * Catapult is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Catapult is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Catapult.  If not, see <http://www.gnu.org/licenses/>.
 */

import CatapultProxy from '../../../src/plugins/rosetta/CatapultProxy.js';
import { RosettaErrorFactory } from '../../../src/plugins/rosetta/rosettaUtils.js';
import { expect } from 'chai';
import sinon from 'sinon';

describe('CatapultProxy', () => {
	const TEST_ENDPOINT = 'http://localhost:3456';

	const assertAsyncErrorThrown = async (func, expectedRosettaError) => {
		try {
			await func();
			expect.fail(`no error thrown - expected ${expectedRosettaError.message}`);
		} catch (err) {
			expect(err.apiError).deep.equal(expectedRosettaError.apiError);
		}
	};

	const stubFetchResult = (urlPath, ok, jsonResult) => {
		if (!global.fetch.restore)
			sinon.stub(global, 'fetch');

		global.fetch.withArgs(`${TEST_ENDPOINT}/${urlPath}`).returns(Promise.resolve({
			ok,
			json: () => jsonResult
		}));
	};

	afterEach(() => {
		if (global.fetch.restore)
			global.fetch.restore();
	});

	describe('fetch', () => {
		it('fails when fetch fails (headers)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('custom/route', false, { foo: 123, bar: 246 });

			// Act + Assert:
			await assertAsyncErrorThrown(() => proxy.fetch('custom/route'), RosettaErrorFactory.CONNECTION_ERROR);

			expect(global.fetch.callCount).to.equal(1);
		});

		it('fails when fetch fails (body)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('custom/route', true, Promise.reject(Error('fetch failed')));

			// Act + Assert:
			await assertAsyncErrorThrown(() => proxy.fetch('custom/route'), RosettaErrorFactory.CONNECTION_ERROR);

			expect(global.fetch.callCount).to.equal(1);
		});

		const assertFetchCalls = expectedResponseOptions => {
			expect(global.fetch.callCount).to.equal(1);
			expect(global.fetch.withArgs(`${TEST_ENDPOINT}/custom/route`, expectedResponseOptions).callCount).to.equal(1);
		};

		it('returns valid response on success (without projection)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('custom/route', true, { foo: 123, bar: 246 });

			// Act:
			const result = await proxy.fetch('custom/route');

			// Assert:
			assertFetchCalls({});
			expect(result).to.deep.equal({ foo: 123, bar: 246 });
		});

		it('returns valid response on success (with projection)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('custom/route', true, { foo: 123, bar: 246 });

			// Act:
			const result = await proxy.fetch('custom/route', jsonObject => jsonObject.bar);

			// Assert:
			assertFetchCalls({});
			expect(result).to.equal(246);
		});

		it('forwards request options to underlying fetch', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('custom/route', true, { foo: 123, bar: 246 });

			// Act:
			const result = await proxy.fetch('custom/route', undefined, { method: 'POST' });

			// Assert:
			assertFetchCalls({ method: 'POST' });
			expect(result).to.deep.equal({ foo: 123, bar: 246 });
		});
	});

	describe('fetchAll', () => {
		const TEST_DATA = [
			{ foo: 2, bar: 5 },
			{ foo: 4, bar: 4 },
			{ foo: 6, bar: 3 },
			{ foo: 8, bar: 2 },
			{ foo: 10, bar: 1 },
			{ foo: 12, bar: 1 },
			{ foo: 14, bar: 2 },
			{ foo: 16, bar: 3 },
			{ foo: 18, bar: 4 },
			{ foo: 20, bar: 5 },
			{ foo: 22, bar: 5 },
			{ foo: 24, bar: 4 }
		];

		it('fails when fetch fails (headers)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('custom/route?pageNumber=1&pageSize=5', true, TEST_DATA.slice(0, 5));
			stubFetchResult('custom/route?pageNumber=2&pageSize=5', false, TEST_DATA.slice(5, 10));
			stubFetchResult('custom/route?pageNumber=3&pageSize=5', true, TEST_DATA.slice(10));

			// Act + Assert:
			await assertAsyncErrorThrown(() => proxy.fetchAll('custom/route', 5), RosettaErrorFactory.CONNECTION_ERROR);

			expect(global.fetch.callCount).to.equal(2);
		});

		it('fails when fetch fails (body)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('custom/route?pageNumber=1&pageSize=5', true, TEST_DATA.slice(0, 5));
			stubFetchResult('custom/route?pageNumber=2&pageSize=5', true, Promise.reject(Error('fetch failed')));
			stubFetchResult('custom/route?pageNumber=3&pageSize=5', true, TEST_DATA.slice(10));

			// Act + Assert:
			await assertAsyncErrorThrown(() => proxy.fetchAll('custom/route', 5), RosettaErrorFactory.CONNECTION_ERROR);

			expect(global.fetch.callCount).to.equal(2);
		});

		const addBasicFetchAllSuccessTests = (urlPath, expectedUrlPathPrefix) => {
			const assertFetchCalls = expectedResponseOptions => {
				const makeUrlForPage = pageNumber => `${TEST_ENDPOINT}/${expectedUrlPathPrefix}pageNumber=${pageNumber}&pageSize=5`;
				expect(global.fetch.callCount).to.equal(3);
				expect(global.fetch.withArgs(makeUrlForPage(1), expectedResponseOptions).callCount).to.equal(1);
				expect(global.fetch.withArgs(makeUrlForPage(2), expectedResponseOptions).callCount).to.equal(1);
				expect(global.fetch.withArgs(makeUrlForPage(3), expectedResponseOptions).callCount).to.equal(1);
			};

			it('returns valid response on success (without projection)', async () => {
				// Arrange:
				const proxy = new CatapultProxy(TEST_ENDPOINT);
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=1&pageSize=5`, true, TEST_DATA.slice(0, 5));
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=2&pageSize=5`, true, TEST_DATA.slice(5, 10));
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=3&pageSize=5`, true, TEST_DATA.slice(10));

				// Act:
				const result = await proxy.fetchAll(urlPath, 5);

				// Assert:
				assertFetchCalls({});
				expect(result).to.deep.equal(TEST_DATA);
			});

			it('returns valid response on success (with projection)', async () => {
				// Arrange:
				const proxy = new CatapultProxy(TEST_ENDPOINT);
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=1&pageSize=5`, true, TEST_DATA.slice(0, 5));
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=2&pageSize=5`, true, TEST_DATA.slice(5, 10));
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=3&pageSize=5`, true, TEST_DATA.slice(10));

				// Act:
				const result = await proxy.fetchAll(urlPath, 5, jsonObject => jsonObject.bar);

				// Assert:
				assertFetchCalls({});
				expect(result).to.deep.equal(TEST_DATA.map(obj => obj.bar));
			});

			it('forwards request options to underlying fetch', async () => {
				// Arrange:
				const proxy = new CatapultProxy(TEST_ENDPOINT);
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=1&pageSize=5`, true, TEST_DATA.slice(0, 5));
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=2&pageSize=5`, true, TEST_DATA.slice(5, 10));
				stubFetchResult(`${expectedUrlPathPrefix}pageNumber=3&pageSize=5`, true, TEST_DATA.slice(10));

				// Act:
				const result = await proxy.fetchAll(urlPath, 5, undefined, { method: 'POST' });

				// Assert:
				assertFetchCalls({ method: 'POST' });
				expect(result).to.deep.equal(TEST_DATA);
			});
		};

		describe('with no query params', () => {
			addBasicFetchAllSuccessTests('custom/route', 'custom/route?');
		});

		describe('with query params', () => {
			addBasicFetchAllSuccessTests('custom/route?q=alpha', 'custom/route?q=alpha&');
		});
	});

	const runGlobalCacheQueryTest = async (action, expectedResult) => {
		// Arrange:
		const proxy = new CatapultProxy(TEST_ENDPOINT);
		stubFetchResult('node/info', true, { node: 'alpha' });
		stubFetchResult('network/properties', true, { network: 'beta' });

		// Act:
		const result = await action(proxy);

		// Assert: only initial calls were made
		expect(global.fetch.callCount).to.equal(2);
		expect(result).to.deep.equal(expectedResult);
	};

	const runGlobalCacheErrorRetryTest = async (failureUrlPath, action, expectedResult) => {
		// Arrange:
		const proxy = new CatapultProxy(TEST_ENDPOINT);
		stubFetchResult('node/info', 'node/info' !== failureUrlPath, { node: 'alpha' });
		stubFetchResult('network/properties', 'network/properties' !== failureUrlPath, { network: 'beta' });

		// Sanity:
		assertAsyncErrorThrown(() => action(proxy), RosettaErrorFactory.CONNECTION_ERROR);

		// Arrange:
		stubFetchResult('node/info', true, { node: 'alpha' });
		stubFetchResult('network/properties', true, { network: 'beta' });

		// Act:
		const result = await action(proxy);

		// Assert: first (failure) and second (success) calls were made
		expect(global.fetch.callCount).to.equal(4);
		expect(result).to.deep.equal(expectedResult);
	};

	describe('nodeInfo', () => {
		it('can retrieve', () => runGlobalCacheQueryTest(proxy => proxy.nodeInfo(), { node: 'alpha' }));

		it('can retrieve (cached)', () => runGlobalCacheQueryTest(async proxy => {
			await proxy.nodeInfo();
			await proxy.nodeInfo();
			return proxy.nodeInfo();
		}, { node: 'alpha' }));

		it('can retrieve after failure', () => runGlobalCacheErrorRetryTest('node/info', proxy => proxy.nodeInfo(), { node: 'alpha' }));
	});

	describe('networkProperties', () => {
		it('can retrieve', () => runGlobalCacheQueryTest(proxy => proxy.networkProperties(), { network: 'beta' }));

		it('can retrieve (cached)', () => runGlobalCacheQueryTest(async proxy => {
			await proxy.networkProperties();
			await proxy.networkProperties();
			return proxy.networkProperties();
		}, { network: 'beta' }));

		it('can retrieve after failure', () => runGlobalCacheErrorRetryTest(
			'network/properties',
			proxy => proxy.networkProperties(),
			{ network: 'beta' }
		));
	});

	describe('resolveMosaicId', () => {
		it('can resolve resolved mosaic id', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);

			// Act:
			const mosaicId = await proxy.resolveMosaicId(0x1234567890ABCDEFn);

			// Assert:
			expect(mosaicId).to.equal(0x1234567890ABCDEFn);
		});

		it('can resolve unresolved mosaic id without location', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('namespaces/9234567890ABCDEF', true, { namespace: { alias: { mosaicId: '1234567890ABCDEF' } } });

			// Act:
			const mosaicId = await proxy.resolveMosaicId(0x9234567890ABCDEFn);

			// Assert:
			expect(mosaicId).to.equal(0x1234567890ABCDEFn);
		});

		it('fails when fetch fails (namespaces)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('namespaces/9234567890ABCDEF', false, { namespace: { alias: { mosaicId: '1234567890ABCDEF' } } });

			// Act + Assert:
			await assertAsyncErrorThrown(() => proxy.resolveMosaicId(0x9234567890ABCDEFn), RosettaErrorFactory.CONNECTION_ERROR);
		});

		const makeResolutionStatement = (unresolved, resolutionEntries) => ({ statement: { unresolved, resolutionEntries } });
		const makeResolutionEntry = (resolved, primaryId, secondaryId) => ({ resolved, source: { primaryId, secondaryId } });

		it('cannot resolve unresolved mosaic id with location when no matching statements exist', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('statements/resolutions/mosaic?height=1234', true, {
				data: [
					makeResolutionStatement('AAAAAAAAAAAAAAAA', [
						makeResolutionEntry('BBBBBBBBBBBBBBBB', 1, 0)
					])
				]
			});

			// Act + Assert:
			await assertAsyncErrorThrown(
				() => proxy.resolveMosaicId(0x9234567890ABCDEFn, { height: 1234n, primaryId: 2, secondaryId: 3 }),
				RosettaErrorFactory.INTERNAL_SERVER_ERROR
			);
		});

		it('cannot resolve unresolved mosaic id with location when no matching resolution entries exist', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('statements/resolutions/mosaic?height=1234', true, {
				data: [
					makeResolutionStatement('9234567890ABCDEF', [
						makeResolutionEntry('1234567890ABCDEF', 2, 4),
						makeResolutionEntry('2234567890ABCDEF', 3, 0)
					])
				]
			});

			// Act + Assert:
			await assertAsyncErrorThrown(
				() => proxy.resolveMosaicId(0x9234567890ABCDEFn, { height: 1234n, primaryId: 2, secondaryId: 3 }),
				RosettaErrorFactory.INTERNAL_SERVER_ERROR
			);
		});

		it('can resolve unresolved mosaic id with location when matching resolution entries exist', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('statements/resolutions/mosaic?height=1234', true, {
				data: [
					makeResolutionStatement('9234567890ABCDEF', [
						makeResolutionEntry('0234567890ABCDEF', 1, 3),
						makeResolutionEntry('1234567890ABCDEF', 2, 2),
						makeResolutionEntry('2234567890ABCDEF', 3, 0)
					])
				]
			});

			// Act:
			const mosaicId = await proxy.resolveMosaicId(0x9234567890ABCDEFn, { height: 1234n, primaryId: 2, secondaryId: 3 });

			// Assert:
			expect(mosaicId).to.equal(0x1234567890ABCDEFn);
		});

		it('fails when fetch fails (statements)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('statements/resolutions/mosaic?height=1234', false, {
				data: [
					makeResolutionStatement('9234567890ABCDEF', [
						makeResolutionEntry('0234567890ABCDEF', 1, 3),
						makeResolutionEntry('1234567890ABCDEF', 2, 2),
						makeResolutionEntry('2234567890ABCDEF', 3, 0)
					])
				]
			});

			// Act + Assert:
			await assertAsyncErrorThrown(
				() => proxy.resolveMosaicId(0x9234567890ABCDEFn, { height: 1234n, primaryId: 2, secondaryId: 3 }),
				RosettaErrorFactory.CONNECTION_ERROR
			);
		});
	});

	describe('mosaicProperties', () => {
		it('can retrieve properties for mosaic with name', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('mosaics/1234567890ABCDEF', true, { mosaic: { divisibility: 3 } });
			stubFetchResult('namespaces/mosaic/names', true, { mosaicNames: [{ names: ['alpha', 'beta'] }] });

			// Act:
			const mosaicProperties = await proxy.mosaicProperties(0x1234567890ABCDEFn);

			// Assert:
			expect(mosaicProperties).to.deep.equal({
				name: 'alpha',
				divisibility: 3
			});
		});

		it('can retrieve properties for mosaic without name', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('mosaics/1234567890ABCDEF', true, { mosaic: { divisibility: 3 } });
			stubFetchResult('namespaces/mosaic/names', true, { mosaicNames: [{ names: [] }] });

			// Act:
			const mosaicProperties = await proxy.mosaicProperties(0x1234567890ABCDEFn);

			// Assert:
			expect(mosaicProperties).to.deep.equal({
				name: '1234567890ABCDEF',
				divisibility: 3
			});
		});

		it('can retrieve properties for mosaic with name (cached)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('mosaics/1234567890ABCDEF', true, { mosaic: { divisibility: 3 } });
			stubFetchResult('namespaces/mosaic/names', true, { mosaicNames: [{ names: ['alpha', 'beta'] }] });

			// Act:
			await proxy.mosaicProperties(0x1234567890ABCDEFn);
			await proxy.mosaicProperties(0x1234567890ABCDEFn);
			const mosaicProperties = await proxy.mosaicProperties(0x1234567890ABCDEFn);

			// Assert: only initial calls were made
			expect(global.fetch.callCount).to.equal(2);
			expect(mosaicProperties).to.deep.equal({
				name: 'alpha',
				divisibility: 3
			});
		});

		it('fails when fetch fails (mosaics)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('mosaics/1234567890ABCDEF', false, { mosaic: { divisibility: 3 } });
			stubFetchResult('namespaces/mosaic/names', true, { mosaicNames: [{ names: ['alpha', 'beta'] }] });

			// Act + Assert:
			await assertAsyncErrorThrown(() => proxy.mosaicProperties(0x1234567890ABCDEFn), RosettaErrorFactory.CONNECTION_ERROR);
		});

		it('fails when fetch fails (namespaces/mosaic/names)', async () => {
			// Arrange:
			const proxy = new CatapultProxy(TEST_ENDPOINT);
			stubFetchResult('mosaics/1234567890ABCDEF', true, { mosaic: { divisibility: 3 } });
			stubFetchResult('namespaces/mosaic/names', false, { mosaicNames: [{ names: ['alpha', 'beta'] }] });

			// Act + Assert:
			await assertAsyncErrorThrown(() => proxy.mosaicProperties(0x1234567890ABCDEFn), RosettaErrorFactory.CONNECTION_ERROR);
		});
	});
});
