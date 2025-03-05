const nextConfig = {
  output: 'export',
  images: {
    unoptimized: true,
  },
  // 使用空字符串作为assetPrefix，表示使用根路径
  assetPrefix: '',
  // 保持URL一致性
  trailingSlash: false,
  basePath: '',
  webpack: (config: { resolve: { fallback: any; }; }) => {
    config.resolve.fallback = {
      ...(config.resolve.fallback || {}),
      fs: false,
      path: false,
    };
    return config;
  },
};

module.exports = nextConfig;