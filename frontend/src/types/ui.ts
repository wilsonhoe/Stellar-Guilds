import React from 'react'

// UI Component Types
export type ButtonVariant = 'primary' | 'outline' | 'ghost' | 'danger'
export type ButtonSize = 'sm' | 'md' | 'lg'

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant
  size?: ButtonSize
  isLoading?: boolean
  leftIcon?: React.ReactNode
  rightIcon?: React.ReactNode
}

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string
  error?: string
  helperText?: string
}

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  title?: string
  description?: string
  footer?: React.ReactNode
}

export interface ModalProps {
  isOpen: boolean
  onClose: () => void
  title?: string
  children: React.ReactNode
  size?: 'sm' | 'md' | 'lg' | 'xl'
}

// Layout Types
export interface SidebarItem {
  id: string
  label: string
  href: string
  icon: React.ReactNode
  isActive?: boolean
}

export interface NavigationItem {
  id: string
  label: string
  href: string
  isActive?: boolean
}

// Mock Data Types
export interface Guild {
  id: string
  name: string
  description: string
  tier: 'bronze' | 'silver' | 'gold' | 'platinum'
  memberCount: number
  reputation: number
  createdAt: string
  logo?: string
}

export interface Bounty {
  id: string
  title: string
  description: string
  reward: {
    amount: number
    currency: string
  }
  deadline: string
  status: 'open' | 'in-progress' | 'completed' | 'expired'
  guildId: string
  createdAt: string
}

export interface UserProfile {
  id: string
  username: string
  email: string
  avatar?: string
  reputation: number
  joinedAt: string
  guilds: string[]
}

// Governance Types
export type ProposalType = 'treasury' | 'rule-change' | 'membership' | 'general'
export type ProposalStatus = 'draft' | 'active' | 'passed' | 'rejected' | 'executed'
export type VoteChoice = 'for' | 'against' | 'abstain'

export interface Vote {
  id: string
  proposalId: string
  voterId: string
  voterAddress: string
  choice: VoteChoice
  votingPower: number
  timestamp: string
}

export interface VotingStats {
  for: number
  against: number
  abstain: number
  total: number
  forPower: number
  againstPower: number
  abstainPower: number
  totalPower: number
}

export interface Proposal {
  id: string
  guildId: string
  title: string
  description: string
  type: ProposalType
  status: ProposalStatus
  proposerId: string
  proposerAddress: string
  proposerName: string
  createdAt: string
  startDate: string
  endDate: string
  quorum: number
  quorumThreshold: number
  executionData?: string
  votes: Vote[]
  votingStats: VotingStats
}

// Dropzone Types
export type DropzoneVariant = 'avatar' | 'banner'

export interface DropzoneProps {
  variant?: DropzoneVariant
  value?: string | null
  onChange?: (url: string | null) => void
  onError?: (error: string) => void
  disabled?: boolean
  className?: string
}

export interface FileValidationResult {
  isValid: boolean
  error?: string
}

export interface UploadResult {
  success: boolean
  url?: string
  error?: string
}

export interface Tab {
  id: string;
  label: string;
  icon?: React.ReactNode;
  content: React.ReactNode;
}

export interface TabsProps {
  tabs: Tab[];
  defaultTab?: string;
  className?: string;
}