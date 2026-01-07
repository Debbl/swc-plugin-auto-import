import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  typescript: { ignoreBuildErrors: true },
  experimental: {
    swcPlugins: [
      [
        "swc-plugin-auto-import",
        {
          presets: ["react"],
          // Set to true to enable debug logging
          debug: false,
        },
      ],
    ],
  },
};

export default nextConfig;
