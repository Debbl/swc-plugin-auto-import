import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  typescript: { ignoreBuildErrors: true },
  experimental: {
    swcPlugins: [
      [
        'swc-plugin-auto-import',
        {
          imports: [
            'react',
            'react-dom',
            {
              "twl": ["cn"]
            },
            {
              "@/utils": ["add"],
            },
            {
              from: 'motion/react-m',
              imports: [['*', 'motion']],
            },
          ],
          // Set to true to enable debug logging
          debug: true,
        },
      ],
    ],
  },
}

export default nextConfig
