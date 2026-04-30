/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  /* 
    This tells Vercel to explicitly include these static files 
    in the serverless function bundle for the proof API.
  */
  outputFileTracingIncludes: {
    '/api/generate-proof': ['./public/circuits/**/*'],
  },
}

module.exports = nextConfig

