/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  generateBuildId: async () => null,
  outputFileTracingIncludes: {
    '/api/generate-proof': ['./public/circuits/**/*'],
  },
}

module.exports = nextConfig

