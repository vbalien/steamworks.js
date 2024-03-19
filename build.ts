import path from 'node:path'
import { mkdirSync, copyFileSync } from 'node:fs'
import { spawn } from 'node:child_process'

const targets = {
    ['x86_64-pc-windows-msvc']: {
        folder: 'win64',
        files: ['steam_api64.dll', 'steam_api64.lib'],
        platform: 'win32',
        arch: 'x64',
    },
    ['x86_64-unknown-linux-gnu']: {
        folder: 'linux64',
        files: ['libsteam_api.so'],
        platform: 'linux',
        arch: 'x64',
    },
    ['x86_64-apple-darwin']: {
        folder: 'osx',
        files: ['libsteam_api.dylib'],
        platform: 'darwin',
        arch: 'x64',
    },
    ['aarch64-apple-darwin']: {
        folder: 'osx',
        files: ['libsteam_api.dylib'],
        platform: 'darwin',
        arch: 'arm64',
    },
}

const target: (typeof targets)[keyof typeof targets] =
    targets[process.argv.at(-1) ?? ''] || Object.values(targets).find((t) => t.platform === process.platform && t.arch === process.arch)

const dist = path.join(__dirname, 'dist', target.folder)
const redist = path.join(__dirname, 'sdk/redistributable_bin', target.folder)
target.files.forEach((file) => {
    const [source, dest] = [path.join(redist, file), path.join(dist, file)]
    try {
        mkdirSync(path.dirname(dest), { recursive: true })
    } catch {}
    copyFileSync(source, dest)
})

const relative = path.relative(process.cwd(), dist)
const params = [
    'build',
    '--platform',
    '--no-dts-header',
    '--no-const-enum',
    '--js',
    'false',
    '--dts',
    '../../client.d.ts',
    relative,
    process.argv.slice(2).join(' '),
]

spawn('napi', params, { stdio: 'inherit', shell: true }).on('exit', (err) => {
    if (err) {
        throw err
    }
})
