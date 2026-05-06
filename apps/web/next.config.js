/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  generateBuildId: async () => null,
  experimental: {
    serverComponentsExternalPackages: [
      '@qvac/sdk', 
      'bare-runtime', 
      'bare-runtime-linux-x64',
      'snarkjs',
      'ffjavascript'
    ],
  },
  outputFileTracingIncludes: {
    '/api/generate-proof': ['./public/circuits/**/*'],
    '/api/ocr': ['./qvac-data/**/*'],
  },
}

module.exports = nextConfig
