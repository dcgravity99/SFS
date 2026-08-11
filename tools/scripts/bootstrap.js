/* ============================================================================
 * Siragugal Film Studio
 * Copyright (C) 2026 Siragugal Film Studio Contributors
 *
 * Licensed under the Apache License, Version 2.0 or the MIT License.
 * See LICENSE files in project root for full terms.
 * ============================================================================ */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('🎬 Initializing Siragugal Film Studio Developer Workspace...');

function checkToolchain() {
    console.log('Checking developer toolchain versions...');
    try {
        const nodeVersion = process.version;
        console.log(`✓ Node.js version: ${nodeVersion}`);

        const rustcVersion = execSync('rustc --version', { encoding: 'utf8' }).trim();
        console.log(`✓ Rust toolchain: ${rustcVersion}`);

        const gitVersion = execSync('git --version', { encoding: 'utf8' }).trim();
        console.log(`✓ Git version: ${gitVersion}`);
    } catch (err) {
        console.error('❌ Error verifying toolchain dependencies:', err.message);
        process.exit(1);
    }
}

function verifyWorkspaceFiles() {
    console.log('Verifying core monorepo workspace files...');
    const requiredFiles = [
        'CONSTITUTION.md',
        'README.md',
        'LICENSE-APACHE',
        'LICENSE-MIT',
        'pnpm-workspace.yaml',
        'Cargo.toml',
        'package.json'
    ];

    const rootDir = path.resolve(__dirname, '../../');
    for (const file of requiredFiles) {
        const filePath = path.join(rootDir, file);
        if (!fs.existsSync(filePath)) {
            console.error(`❌ Missing required workspace file: ${file}`);
            process.exit(1);
        }
    }
    console.log('✓ All workspace configuration files verified.');
}

function main() {
    checkToolchain();
    verifyWorkspaceFiles();
    console.log('✅ Workspace bootstrap complete! Ready for development.');
}

main();
