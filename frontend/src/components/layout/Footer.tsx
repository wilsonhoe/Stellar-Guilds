"use client";

import React from "react";
import Link from "next/link";
import { Github, Twitter, MessageCircle, FileText, Shield } from "lucide-react";
import { cn } from "@/lib/utils";

interface FooterProps {
  className?: string;
}

const socialLinks = [
  {
    name: "Twitter",
    href: "https://twitter.com/stellarguilds",
    icon: Twitter,
    label: "Follow us on Twitter",
  },
  {
    name: "Discord",
    href: "https://discord.gg/stellarguilds",
    icon: MessageCircle,
    label: "Join our Discord",
  },
  {
    name: "GitHub",
    href: "https://github.com/stellar-guilds",
    icon: Github,
    label: "View our GitHub",
  },
];

const legalLinks = [
  {
    name: "Privacy Policy",
    href: "/privacy",
    icon: Shield,
  },
  {
    name: "Terms of Service",
    href: "/terms",
    icon: FileText,
  },
];

const Footer = ({ className }: FooterProps) => {
  const currentYear = new Date().getFullYear();

  return (
    <footer
      className={cn(
        "border-t border-stellar-lightNavy bg-stellar-darkNavy py-6 mt-auto",
        className,
      )}
    >
      <div className="container px-4 mx-auto max-w-7xl">
        <div className="flex flex-col md:flex-row justify-between items-center gap-4">
          <div className="text-stellar-slate text-sm text-center md:text-left">
            © {currentYear} Stellar Guilds. Built on the Stellar Network.
          </div>

          <div className="flex items-center gap-4">
            {legalLinks.map((link) => (
              <Link
                key={link.name}
                href={link.href}
                className="flex items-center gap-1.5 text-stellar-slate hover:text-stellar-lightSlate transition-colors text-sm"
              >
                <link.icon size={14} />
                <span className="hidden sm:inline">{link.name}</span>
              </Link>
            ))}
          </div>

          <div className="flex items-center gap-4">
            {socialLinks.map((link) => (
              <a
                key={link.name}
                href={link.href}
                target="_blank"
                rel="noopener noreferrer"
                className="text-stellar-slate hover:text-stellar-lightSlate transition-colors"
                aria-label={link.label}
              >
                <link.icon size={18} />
              </a>
            ))}
          </div>
        </div>
      </div>
    </footer>
  );
};

export { Footer };
