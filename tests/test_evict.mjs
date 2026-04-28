import { markdownToPdf, evict } from "../index.js";

const doc = "# Hello\n\nA short paragraph with **bold** and *italic* text.";

async function main() {
  // --- Test 1: evict accepts various maxAge values without throwing ---
  console.log("Test: evict accepts maxAge parameter");
  evict(0);
  evict(1);
  evict(100);
  console.log("  PASS");

  // --- Test 2: evict does not affect conversion correctness ---
  console.log("Test: evict does not affect conversion correctness");
  const pdfBefore = await markdownToPdf(doc);
  evict(0);
  const pdfAfter = await markdownToPdf(doc);

  if (pdfBefore.length === 0) throw new Error("PDF before evict is empty");
  if (pdfAfter.length === 0) throw new Error("PDF after evict is empty");
  if (pdfBefore.length !== pdfAfter.length) {
    throw new Error(
      `PDF size changed after evict: ${pdfBefore.length} -> ${pdfAfter.length}`,
    );
  }
  console.log("  PASS");

  // --- Test 3: evict reclaims native memory ---
  console.log("Test: evict reclaims native memory");

  // Warm up
  await markdownToPdf(doc);
  evict(0);
  if (global.gc) global.gc();

  const before = process.memoryUsage();

  for (let i = 0; i < 200; i++) {
    await markdownToPdf(doc);
  }
  if (global.gc) global.gc();
  const afterNoEvict = process.memoryUsage();
  const rssGrowthNoEvict = afterNoEvict.rss - before.rss;

  // Reset
  evict(0);
  if (global.gc) global.gc();
  const baseline2 = process.memoryUsage();

  for (let i = 0; i < 200; i++) {
    await markdownToPdf(doc);
    evict(0);
  }
  if (global.gc) global.gc();
  const afterEvict = process.memoryUsage();
  const rssGrowthWithEvict = afterEvict.rss - baseline2.rss;

  console.log(
    `  RSS growth without evict: ${(rssGrowthNoEvict / 1024).toFixed(0)} KB`,
  );
  console.log(
    `  RSS growth with evict:    ${(rssGrowthWithEvict / 1024).toFixed(0)} KB`,
  );

  if (rssGrowthWithEvict >= rssGrowthNoEvict) {
    throw new Error(
      `Expected evict to reduce memory growth. ` +
        `Without evict: ${(rssGrowthNoEvict / 1024).toFixed(0)} KB, ` +
        `with evict: ${(rssGrowthWithEvict / 1024).toFixed(0)} KB`,
    );
  }
  console.log("  PASS");

  console.log("\nAll evict tests passed.");
}

main();
