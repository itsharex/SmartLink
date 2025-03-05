import React from 'react';
import Link from 'next/link';
import { FaFacebook, FaTwitter, FaLinkedin, FaInstagram } from 'react-icons/fa';

const Footer: React.FC = () => {
  return (
    <footer className="bg-bg-primary/80 pt-20 pb-10 relative">
      <div className="container mx-auto px-6">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-10">
          {/* Brand */}
          <div className="space-y-6">
            <Link
              href="/"
              className="flex items-center text-2xl text-text-primary font-orbitron font-bold"
            >
              <div className="relative mr-3">
                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                  <path d="M12 2L2 7L12 12L22 7L12 2Z" fill="var(--accent-primary)" />
                  <path
                    d="M2 17L12 22L22 17"
                    stroke="var(--accent-secondary)"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  />
                  <path
                    d="M2 12L12 17L22 12"
                    stroke="var(--accent-primary)"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  />
                </svg>
                <div className="absolute inset-0 -m-1 bg-gradient-to-r from-accent-primary to-accent-secondary rounded-full blur-md opacity-70 -z-10" />
              </div>
              SmartLink
            </Link>
            <p className="text-text-secondary">
              SmartLink 是一款集即时通讯、社交互动、AI 助手于一体的智能平台，致力于为用户提供安全、高效、便捷的沟通体验。
            </p>
            <div className="flex space-x-4">
              <a
                href="#"
                className="w-10 h-10 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition"
              >
                <FaFacebook className="w-[18px] h-[18px]" />
              </a>
              <a
                href="#"
                className="w-10 h-10 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition"
              >
                <FaTwitter className="w-[18px] h-[18px]" />
              </a>
              <a
                href="#"
                className="w-10 h-10 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition"
              >
                <FaLinkedin className="w-[18px] h-[18px]" />
              </a>
              <a
                href="#"
                className="w-10 h-10 flex items-center justify-center rounded-full bg-white/5 text-text-primary hover:bg-accent-primary/10 hover:text-accent-primary transition"
              >
                <FaInstagram className="w-[18px] h-[18px]" />
              </a>
            </div>
          </div>

          {/* Links Column 1 */}
          <div>
            <h3 className="text-xl font-semibold mb-6 text-text-primary">产品</h3>
            <ul className="space-y-4">
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  功能
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  下载
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  定价
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  企业版
                </a>
              </li>
            </ul>
          </div>

          {/* Links Column 2 */}
          <div>
            <h3 className="text-xl font-semibold mb-6 text-text-primary">资源</h3>
            <ul className="space-y-4">
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  帮助中心
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  开发者文档
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  API
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  社区论坛
                </a>
              </li>
            </ul>
          </div>

          {/* Links Column 3 */}
          <div>
            <h3 className="text-xl font-semibold mb-6 text-text-primary">公司</h3>
            <ul className="space-y-4">
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  关于我们
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  新闻
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  招聘
                </a>
              </li>
              <li>
                <a
                  href="#"
                  className="text-text-secondary hover:text-accent-primary transition-colors duration-300 hover:translate-x-1 inline-block"
                >
                  联系我们
                </a>
              </li>
            </ul>
          </div>
        </div>

        {/* Bottom Section */}
        <div className="mt-16 pt-8 border-t border-white/10 flex flex-col md:flex-row justify-between items-center">
          <p className="text-text-secondary text-sm">© 2023 SmartLink. 保留所有权利。</p>
          <div className="flex space-x-6 mt-4 md:mt-0">
            <a href="#" className="text-text-secondary text-sm hover:text-accent-primary transition-colors">
              隐私政策
            </a>
            <a href="#" className="text-text-secondary text-sm hover:text-accent-primary transition-colors">
              服务条款
            </a>
            <a href="#" className="text-text-secondary text-sm hover:text-accent-primary transition-colors">
              Cookie 设置
            </a>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;