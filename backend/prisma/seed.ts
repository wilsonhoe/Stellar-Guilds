import { PrismaClient, Prisma } from '@prisma/client';
import { faker } from '@faker-js/faker';

const prisma = new PrismaClient();

async function main() {
  console.log('🛠️  Starting seed script for Stellar Guilds...');

  // Clean existing data in dependency order
  await prisma.bountyPayout.deleteMany();
  await prisma.bountyApplication.deleteMany();
  await prisma.bountyMilestone.deleteMany();
  await prisma.bounty.deleteMany();
  await prisma.guildMembership.deleteMany();
  await prisma.guild.deleteMany();
  await prisma.user.deleteMany();

  // Create 10 dummy users
  console.log('Seeding users...');
  const users = [];
  for (let i = 0; i < 10; i++) {
    const firstName = faker.person.firstName();
    const lastName = faker.person.lastName();
    const user = await prisma.user.create({
      data: {
        firstName,
        lastName,
        username: `${faker.internet.username({ firstName, lastName })}${i}`,
        email: faker.internet.email({ firstName, lastName }),
        password: faker.internet.password(),
        avatarUrl: faker.image.avatar(),
        bio: faker.person.bio(),
        isActive: true,
      },
    });
    users.push(user);
  }

  // Create guilds with owners
  console.log('Seeding guilds...');
  const guilds = await Promise.all([
    prisma.guild.create({
      data: {
        name: 'PrivacyGuard',
        slug: 'privacyguard',
        description: 'Focused on privacy protocols and zero-knowledge research',
        ownerId: users[0].id,
        avatarUrl: 'https://api.dicebear.com/7.x/identicon/svg?seed=privacy',
        bannerUrl: 'https://api.dicebear.com/7.x/identicon/svg?seed=privacy-hero',
      },
    }),
    prisma.guild.create({
      data: {
        name: 'StellarDesign',
        slug: 'stellardesign',
        description: 'Design-first tooling for cross-chain dApps',
        ownerId: users[1].id,
        avatarUrl: 'https://api.dicebear.com/7.x/identicon/svg?seed=design',
        bannerUrl: 'https://api.dicebear.com/7.x/identicon/svg?seed=design-hero',
      },
    }),
    prisma.guild.create({
      data: {
        name: 'DeFi Builders',
        slug: 'defi-builders',
        description: 'Building the future of decentralized finance',
        ownerId: users[2].id,
        avatarUrl: 'https://api.dicebear.com/7.x/identicon/svg?seed=defi',
      },
    }),
  ]);

  // Create memberships
  console.log('Seeding guild memberships...');
  const memberships = [];
  
  // Guild 0: Owner + 4 members
  memberships.push(
    prisma.guildMembership.create({
      data: { userId: users[0].id, guildId: guilds[0].id, role: 'OWNER', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[3].id, guildId: guilds[0].id, role: 'ADMIN', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[4].id, guildId: guilds[0].id, role: 'MEMBER', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[5].id, guildId: guilds[0].id, role: 'MEMBER', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[6].id, guildId: guilds[0].id, role: 'MEMBER', status: 'APPROVED', joinedAt: new Date() },
    }),
  );

  // Guild 1: Owner + 3 members
  memberships.push(
    prisma.guildMembership.create({
      data: { userId: users[1].id, guildId: guilds[1].id, role: 'OWNER', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[4].id, guildId: guilds[1].id, role: 'MODERATOR', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[7].id, guildId: guilds[1].id, role: 'MEMBER', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[8].id, guildId: guilds[1].id, role: 'MEMBER', status: 'APPROVED', joinedAt: new Date() },
    }),
  );

  // Guild 2: Owner + 2 members
  memberships.push(
    prisma.guildMembership.create({
      data: { userId: users[2].id, guildId: guilds[2].id, role: 'OWNER', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[8].id, guildId: guilds[2].id, role: 'MEMBER', status: 'APPROVED', joinedAt: new Date() },
    }),
    prisma.guildMembership.create({
      data: { userId: users[9].id, guildId: guilds[2].id, role: 'MEMBER', status: 'APPROVED', joinedAt: new Date() },
    }),
  );

  await Promise.all(memberships);

  // Create bounties
  console.log('Seeding bounties...');
  const bounties = await Promise.all([
    prisma.bounty.create({
      data: {
        title: 'Implement Zero-Knowledge Proof for Voting',
        description: 'Build a robust ZK-proof system using Circom for a DAO governance module.',
        status: 'OPEN',
        rewardAmount: new Prisma.Decimal(5000),
        rewardToken: 'USDC',
        deadline: new Date(Date.now() + 7 * 24 * 3600 * 1000),
        creatorId: users[0].id,
        guildId: guilds[0].id,
      },
    }),
    prisma.bounty.create({
      data: {
        title: 'Optimize Landing Page Hero Animation',
        description: 'Improve Three.js hero animation performance and add responsive behavior.',
        status: 'IN_PROGRESS',
        rewardAmount: new Prisma.Decimal(1200),
        rewardToken: 'STR',
        deadline: new Date(Date.now() + 14 * 24 * 3600 * 1000),
        creatorId: users[1].id,
        assigneeId: users[4].id,
        guildId: guilds[1].id,
      },
    }),
    prisma.bounty.create({
      data: {
        title: 'Complete Audit for On-Chain Treasury Workflow',
        description: 'Review all smart-contract hooks and treasury flows for edge cases.',
        status: 'COMPLETED',
        rewardAmount: new Prisma.Decimal(7500),
        rewardToken: 'STELLAR',
        deadline: new Date(Date.now() - 2 * 24 * 3600 * 1000),
        creatorId: users[0].id,
        assigneeId: users[3].id,
        guildId: guilds[0].id,
      },
    }),
  ]);

  console.log(`✅ Seed completed: ${users.length} users, ${guilds.length} guilds, ${bounties.length} bounties created.`);
  console.log('🎉 Finished Prisma seed script.');
}

main()
  .catch((error) => {
    console.error('❌ Seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
