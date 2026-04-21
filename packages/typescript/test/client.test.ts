import { afterEach, describe, expect, test } from "bun:test";
import { mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

import {
	configurePogoData,
	createPogoDataClient,
	DEFAULT_BASE_URL,
	datasetDefinitions,
	type FetchImplementation,
	PogoDataError,
	pogoData,
	translationCategories,
	translationLocales,
} from "../src";

const PACKAGE_DIRECTORY = new URL("../", import.meta.url);

afterEach(() => {
	configurePogoData({
		baseUrl: DEFAULT_BASE_URL,
		fetch: undefined,
		headers: undefined,
	});
});

function createJsonResponse(payload: unknown): Response {
	return new Response(JSON.stringify(payload), {
		headers: {
			"content-type": "application/json",
		},
		status: 200,
	});
}

function createFetchSpy(payload: unknown = { ok: true }) {
	const requests: { headers: Headers; url: string }[] = [];
	const fetchImplementation: FetchImplementation = async (input, init) => {
		requests.push({
			headers: new Headers(init?.headers),
			url: typeof input === "string" ? input : input.toString(),
		});

		return createJsonResponse(payload);
	};

	return {
		fetchImplementation,
		requests,
	};
}

function runCommand(
	command: string[],
	cwd: string,
	environment: Record<string, string> = {},
): string {
	const result = Bun.spawnSync(command, {
		cwd,
		env: {
			...process.env,
			...environment,
		},
		stderr: "pipe",
		stdout: "pipe",
	});

	if (result.exitCode !== 0) {
		throw new Error(
			[
				`Command failed: ${command.join(" ")}`,
				new TextDecoder().decode(result.stderr),
			].join("\n"),
		);
	}

	return new TextDecoder().decode(result.stdout).trim();
}

describe("URL building", () => {
	test("uses the default host for collection endpoints", async () => {
		const { fetchImplementation, requests } = createFetchSpy([]);
		const client = createPogoDataClient({ fetch: fetchImplementation });

		await client.pokemon.list();

		expect(requests[0]?.url).toBe(`${DEFAULT_BASE_URL}/pokemon.json`);
		expect(requests[0]?.headers.get("accept")).toBe("application/json");
	});

	test("supports a custom base URL and trims trailing slashes", async () => {
		const { fetchImplementation, requests } = createFetchSpy({});
		const client = createPogoDataClient({
			baseUrl: "https://cdn.example.com/pogo/data/v1///",
			fetch: fetchImplementation,
		});

		await client.moves.get(13);

		expect(requests[0]?.url).toBe(
			"https://cdn.example.com/pogo/data/v1/moves/13.json",
		);
	});

	test("supports string identifiers", async () => {
		const { fetchImplementation, requests } = createFetchSpy({});
		const client = createPogoDataClient({ fetch: fetchImplementation });

		await client.forms.get("1002");

		expect(requests[0]?.url).toBe(`${DEFAULT_BASE_URL}/forms/1002.json`);
	});
});

describe("singleton configuration", () => {
	test("starts with the default host", async () => {
		const { fetchImplementation, requests } = createFetchSpy({});

		configurePogoData({ fetch: fetchImplementation });
		await pogoData.weather.get(1);

		expect(requests[0]?.url).toBe(`${DEFAULT_BASE_URL}/weather/1.json`);
	});

	test("updates the shared singleton after configuration", async () => {
		const { fetchImplementation, requests } = createFetchSpy({});

		configurePogoData({
			baseUrl: "https://mirror.example.com/data/v1/",
			fetch: fetchImplementation,
		});
		await pogoData.teams.get(2);

		expect(requests[0]?.url).toBe(
			"https://mirror.example.com/data/v1/teams/2.json",
		);
	});

	test("keeps separate instance clients isolated from singleton changes", async () => {
		const clientFetch = createFetchSpy({});
		const singletonFetch = createFetchSpy({});
		const client = createPogoDataClient({
			baseUrl: "https://instance.example.com/data/v1",
			fetch: clientFetch.fetchImplementation,
		});

		configurePogoData({
			baseUrl: "https://singleton.example.com/data/v1",
			fetch: singletonFetch.fetchImplementation,
		});

		await client.items.get(1);
		await pogoData.items.get(1);

		expect(clientFetch.requests[0]?.url).toBe(
			"https://instance.example.com/data/v1/items/1.json",
		);
		expect(singletonFetch.requests[0]?.url).toBe(
			"https://singleton.example.com/data/v1/items/1.json",
		);
	});
});

describe("endpoint coverage", () => {
	test("exposes every top-level V1 dataset as list/get methods", () => {
		for (const definition of datasetDefinitions) {
			const resource = pogoData[definition.property];

			expect(resource).toBeDefined();
			expect(typeof resource.list).toBe("function");
			expect(typeof resource.get).toBe("function");
		}
	});

	test("builds locale and category translation paths correctly", async () => {
		const { fetchImplementation, requests } = createFetchSpy({});
		const client = createPogoDataClient({ fetch: fetchImplementation });

		await client.translations.getLocale("en");
		await client.translations.getCategory("en", "misc");

		expect(requests[0]?.url).toBe(`${DEFAULT_BASE_URL}/translations/en.json`);
		expect(requests[1]?.url).toBe(
			`${DEFAULT_BASE_URL}/translations/en/misc.json`,
		);
	});

	test("publishes discovered locale and category metadata", () => {
		expect(translationLocales.length).toBeGreaterThan(0);
		expect(translationCategories.length).toBeGreaterThan(0);
		expect(translationLocales).toContain("en");
		expect(translationCategories).toContain("misc");
	});
});

describe("error handling", () => {
	test("throws a typed error on non-OK responses", async () => {
		const client = createPogoDataClient({
			fetch: async () => new Response("missing", { status: 404 }),
		});

		await expect(client.pokemon.get(999999)).rejects.toBeInstanceOf(
			PogoDataError,
		);

		try {
			await client.pokemon.get(999999);
		} catch (error) {
			expect(error).toBeInstanceOf(PogoDataError);
			expect((error as PogoDataError).status).toBe(404);
		}
	});

	test("throws a typed error on network failures", async () => {
		const client = createPogoDataClient({
			fetch: async () => {
				throw new Error("network down");
			},
		});

		await expect(client.moves.get(13)).rejects.toBeInstanceOf(PogoDataError);
	});

	test("throws a typed error on invalid JSON", async () => {
		const client = createPogoDataClient({
			fetch: async () =>
				new Response("not json", {
					headers: { "content-type": "application/json" },
				}),
		});

		await expect(client.raids.list()).rejects.toBeInstanceOf(PogoDataError);
	});
});

describe("packaging", () => {
	test("builds and installs from the packed tarball", async () => {
		const packageDirectory = PACKAGE_DIRECTORY.pathname;
		const distDirectory = join(packageDirectory, "dist");

		runCommand(["bun", "run", "build"], packageDirectory);

		const tempDirectory = await mkdtemp(join(tmpdir(), "pogo-data-sdk-"));
		const npmCacheDirectory = join(tempDirectory, ".npm-cache");
		const packOutput = runCommand(
			["npm", "pack", "--pack-destination", tempDirectory],
			packageDirectory,
			{ NPM_CONFIG_CACHE: npmCacheDirectory },
		);
		const tarballName = packOutput.split("\n").at(-1);

		if (tarballName === undefined) {
			throw new Error("npm pack did not return a tarball name.");
		}

		const installDirectory = join(tempDirectory, "install-check");
		await mkdir(installDirectory, { recursive: true });
		await writeFile(
			join(installDirectory, "package.json"),
			JSON.stringify({ name: "install-check", private: true, type: "module" }),
		);

		runCommand(
			["npm", "install", "--ignore-scripts", join(tempDirectory, tarballName)],
			installDirectory,
			{ NPM_CONFIG_CACHE: npmCacheDirectory },
		);

		const esmCheck = runCommand(
			[
				"node",
				"--input-type=module",
				"--eval",
				"const mod = await import('@watwowmap/pogo-data'); if (typeof mod.createPogoDataClient !== 'function') { throw new Error('Missing createPogoDataClient export'); } if (!('pogoData' in mod)) { throw new Error('Missing pogoData export'); }",
			],
			installDirectory,
		);
		const cjsCheck = runCommand(
			[
				"node",
				"--eval",
				"const mod = require('@watwowmap/pogo-data'); if (typeof mod.createPogoDataClient !== 'function') { throw new Error('Missing createPogoDataClient export'); }",
			],
			installDirectory,
		);

		expect(esmCheck).toBe("");
		expect(cjsCheck).toBe("");
		expect(await readFile(join(distDirectory, "index.d.ts"), "utf8")).toContain(
			"interface PogoDataClient",
		);

		await rm(tempDirectory, { force: true, recursive: true });
	});
});
