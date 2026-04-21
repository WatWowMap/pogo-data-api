import { mkdir, rm } from "node:fs/promises";
import Bun from "bun";
import { generate } from "pogo-data-generator";

interface DataRecord {
	[key: string]: unknown;
}

interface FileWriteTask {
	path: URL;
	contents: string;
}

const V1_DIRECTORY = new URL("./v1/", import.meta.url);
const WRITE_CONCURRENCY = 64;

function isRecord(value: unknown): value is DataRecord {
	return typeof value === "object" && value !== null && !Array.isArray(value);
}

function toKebabCase(value: string): string {
	return value
		.replace(/([a-z0-9])([A-Z])/g, "$1-$2")
		.replace(/[^a-zA-Z0-9]+/g, "-")
		.replace(/-+/g, "-")
		.replace(/^-|-$/g, "")
		.toLowerCase();
}

async function runWithConcurrency<T>(
	items: readonly T[],
	concurrency: number,
	worker: (item: T) => Promise<void>,
): Promise<void> {
	let currentIndex = 0;

	const workers = Array.from(
		{ length: Math.min(concurrency, items.length) },
		async () => {
			while (true) {
				const item = items[currentIndex];
				currentIndex += 1;

				if (item === undefined) {
					return;
				}

				await worker(item);
			}
		},
	);

	await Promise.all(workers);
}

export async function writeApiData(data: DataRecord): Promise<void> {
	const directoryTasks: URL[] = [];
	const fileWriteTasks: FileWriteTask[] = [];

	for (const [categoryKey, categoryValue] of Object.entries(data)) {
		if (!isRecord(categoryValue)) {
			throw new TypeError(
				`Expected "${categoryKey}" to be an object of keyed values.`,
			);
		}

		const categorySlug = toKebabCase(categoryKey);
		const categoryDirectory = new URL(`${categorySlug}/`, V1_DIRECTORY);
		const categoryEntries = Object.entries(categoryValue);

		directoryTasks.push(categoryDirectory);
		if (categoryKey !== "translations") {
			fileWriteTasks.push({
				path: new URL(`${categorySlug}.json`, V1_DIRECTORY),
				contents: JSON.stringify(
					categoryEntries.map(([, value]) => value),
					null,
					2,
				),
			});
		}

		for (const [entityKey, entityValue] of categoryEntries) {
			fileWriteTasks.push({
				path: new URL(`${toKebabCase(entityKey)}.json`, categoryDirectory),
				contents: JSON.stringify(entityValue, null, 2),
			});

			if (categoryKey !== "translations" || !isRecord(entityValue)) {
				continue;
			}

			const entityDirectory = new URL(
				`${toKebabCase(entityKey)}/`,
				categoryDirectory,
			);

			directoryTasks.push(entityDirectory);

			for (const [nestedKey, nestedValue] of Object.entries(entityValue)) {
				fileWriteTasks.push({
					path: new URL(`${toKebabCase(nestedKey)}.json`, entityDirectory),
					contents: JSON.stringify(nestedValue, null, 2),
				});
			}
		}
	}

	await rm(V1_DIRECTORY, { recursive: true, force: true });
	await mkdir(V1_DIRECTORY, { recursive: true });
	await runWithConcurrency(
		directoryTasks,
		WRITE_CONCURRENCY,
		async (directory) => {
			await mkdir(directory, { recursive: true });
		},
	);
	await runWithConcurrency(fileWriteTasks, WRITE_CONCURRENCY, async (task) => {
		await Bun.write(task.path, task.contents);
	});
}

async function main() {
	// The upstream package accepts this runtime template shape, but its exported
	// typings do not fully describe it.
	const template = {
		globalOptions: {
			keyJoiner: "_",
			includeProtos: true,
		},
		pokemon: { enabled: true, template: {} },
		costumes: { enabled: true, template: {} },
		types: { enabled: true, template: {} },
		moves: { enabled: true, template: {} },
		items: {
			enabled: true,
			options: {
				minTrainerLevel: 100,
			},
			template: {},
		},
		questConditions: { enabled: true, template: {} },
		questRewardTypes: { enabled: true, template: {} },
		questTypes: { enabled: true, template: {} },
		invasions: { enabled: true, template: {} },
		weather: { enabled: true, template: {} },
		raids: { enabled: true, template: {} },
		locationCards: { enabled: true, template: {} },
		teams: { enabled: true, template: {} },
		routeTypes: { enabled: true, template: {} },
		translations: {
			enabled: true,
			options: {
				mergeCategories: false,
				useLanguageAsRef: false,
			},
			locales: {
				de: true,
				en: true,
				es: true,
				"es-mx": true,
				fr: true,
				hi: true,
				id: true,
				it: true,
				ja: true,
				ko: true,
				"pt-br": true,
				ru: true,
				th: true,
				"zh-tw": true,
				tr: true,
			},
		},
	} as NonNullable<Parameters<typeof generate>[0]>["template"];

	const data = await generate({
		raw: true,
		template,
	});

	await writeApiData(data);
}

await main();
