import fs from "fs";
import path from "path";

import { markdownToTypstCode, markdownToPdf } from "../index.js";

const numFuzzRegressionTests = 37;

let TEST_FILES = [
  "demo.md",
  "pulldown_cmark_regressions.md",

  // ai-generated:
  "bidir.md",
  "edge_cases.md",
  "syntax.md",
  "html.md",
  "long.md",
  // "advanced_edge_cases.md",
  // "parsing_edge_cases.md",
  "vulnerability_test_cases.md",
  "torture_typst.md",
  "torture_typst_2.md",
  "malformed_md_html.md",
].concat(
  [...new Array(numFuzzRegressionTests)].map((_, i) => `fuzz_${i + 1}.md`),
);

const PROFILE = process.env.PROFILE === "true";
const NUM_RUNS = PROFILE ? 25 : 1;
const WAIT_TIME_BETWEEN_RUNS = 10_000;

const REPORT_SEPARATOR = "\t";

function logMemoryUsage() {
  if (PROFILE) return;

  const memoryUsage = process.memoryUsage();

  console.log("Memory usage:");
  console.log(`  RSS: ${memoryUsage.rss / (1024 * 1024)} MB`); // Resident Set Size
  console.log(`  Heap Total: ${memoryUsage.heapTotal / (1024 * 1024)} MB`); // Total size of the V8 heap
  console.log(`  Heap Used: ${memoryUsage.heapUsed / (1024 * 1024)} MB`); // Actual memory used by V8 heap
  console.log(`  External: ${memoryUsage.external / (1024 * 1024)} MB`); // Memory used by C++ objects bound to JavaScript objects
  console.log(
    `  Array Buffers: ${memoryUsage.arrayBuffers / (1024 * 1024)} MB`,
  ); // Memory allocated for ArrayBuffers and Buffers
}

function getCodePoints(str) {
  const codePoints = [];
  for (let i = 0; i < str.length; ) {
    const codePoint = str.codePointAt(i);
    codePoints.push(
      `U+${codePoint.toString(16).toUpperCase().padStart(4, "0")}`,
    );
    // Handle surrogate pairs (characters > U+FFFF)
    i += codePoint > 0xffff ? 2 : 1;
  }
  return codePoints;
}

function calculateStats(times) {
  if (times.length === 0) return { avg: 0, min: 0, max: 0 };

  const sum = times.reduce((a, b) => a + b, 0);
  const avg = sum / times.length;
  const min = Math.min(...times);
  const max = Math.max(...times);

  return { avg, min, max };
}

async function main() {
  // Store timing data for each test file and stage
  const timingData = {};

  for (const test of TEST_FILES) {
    timingData[test] = {
      typst: [],
      pdf: [],
    };
  }

  // Store memory data
  const memoryData = {
    rss: [],
    external: [],
  };

  for (let i = 0; i < NUM_RUNS; i++) {
    if (!PROFILE && NUM_RUNS > 1) {
      console.log(`Run ${i + 1} of ${NUM_RUNS}`);
      logMemoryUsage();
    }

    for (const test of TEST_FILES) {
      const markdown = fs.readFileSync(
        path.join(import.meta.dirname, test),
        "utf8",
      );
      let typstCode = null;

      try {
        const startTime = performance.now();
        typstCode = markdownToTypstCode(markdown);
        const endTime = performance.now();
        const typstTime = endTime - startTime;
        timingData[test].typst.push(typstTime);

        if (!PROFILE) {
          console.log(`Typst for ${test}: ${typstTime.toFixed(2)}ms`);
        }
        fs.writeFileSync(`${test}.typ`, typstCode);
      } catch (error) {
        if (!PROFILE) {
          console.error(`Error generating Typst for ${test}:`, error);
          if (process.env.CI) {
            throw error;
          }
        }
      }

      // if (test.includes('fuzz')) {
      //     console.log(`markdown: '${markdown}'`)
      //     console.log(`markdown code points: ${getCodePoints(markdown)}`)
      //     console.log(`typst: '${typstCode}'`)
      //     console.log(`typst code points: ${getCodePoints(typstCode)}`)
      // }

      try {
        const startTime = performance.now();
        const pdfBytes = markdownToPdf(markdown);
        const endTime = performance.now();
        const pdfTime = endTime - startTime;
        timingData[test].pdf.push(pdfTime);

        if (!PROFILE) {
          console.log(`PDF for ${test}: ${pdfTime.toFixed(2)}ms`);
        }
        fs.writeFileSync(`${test}.pdf`, pdfBytes);
      } catch (error) {
        if (!PROFILE) {
          console.error(`❌ PDF generation failed for ${test}:`, error);
          console.error("Error details:", error.message);
          if (process.env.CI) {
            throw error;
          }
        }
      }

      if (!PROFILE) {
        logMemoryUsage();
        console.log("-".repeat(50));
      }
    }

    // Collect memory data after each run
    const memoryUsage = process.memoryUsage();
    memoryData.rss.push(memoryUsage.rss / (1024 * 1024)); // Convert to MB
    memoryData.external.push(memoryUsage.external / (1024 * 1024)); // Convert to MB

    if (i < NUM_RUNS - 1 && !PROFILE) {
      await new Promise((resolve) =>
        setTimeout(resolve, WAIT_TIME_BETWEEN_RUNS),
      );
    }
  }

  // Print timing statistics as CSV
  if (PROFILE) {
    console.log(
      `test_file${REPORT_SEPARATOR}stage${REPORT_SEPARATOR}avg_ms${REPORT_SEPARATOR}min_ms${REPORT_SEPARATOR}max_ms`,
    );

    for (const test of TEST_FILES) {
      const typstStats = calculateStats(timingData[test].typst);
      console.log(
        `${test}${REPORT_SEPARATOR}typst${REPORT_SEPARATOR}${typstStats.avg.toFixed(2)}${REPORT_SEPARATOR}${typstStats.min.toFixed(2)}${REPORT_SEPARATOR}${typstStats.max.toFixed(2)}`,
      );

      const pdfStats = calculateStats(timingData[test].pdf);
      console.log(
        `${test}${REPORT_SEPARATOR}pdf${REPORT_SEPARATOR}${pdfStats.avg.toFixed(2)}${REPORT_SEPARATOR}${pdfStats.min.toFixed(2)}${REPORT_SEPARATOR}${pdfStats.max.toFixed(2)}`,
      );
    }

    // Print memory statistics
    console.log(
      `\nmemory_type${REPORT_SEPARATOR}avg_mb${REPORT_SEPARATOR}min_mb${REPORT_SEPARATOR}max_mb`,
    );
    const rssStats = calculateStats(memoryData.rss);
    // console.log(`rss${REPORT_SEPARATOR}${rssStats.avg.toFixed(2)}${REPORT_SEPARATOR}${rssStats.min.toFixed(2)}${REPORT_SEPARATOR}${rssStats.max.toFixed(2)}`);
    console.log(rssStats.avg.toFixed(2));
    console.log(rssStats.min.toFixed(2));
    console.log(rssStats.max.toFixed(2));

    const externalStats = calculateStats(memoryData.external);
    console.log(externalStats.avg.toFixed(2));
    console.log(externalStats.min.toFixed(2));
    console.log(externalStats.max.toFixed(2));
  }
}

main();
