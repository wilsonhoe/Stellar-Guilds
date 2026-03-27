constructor(private prisma: PrismaClient) {}

private generateSlug(name: string): string {
  return name.toLowerCase().replace(/\s+/g, '-');
}

async createGuild(data: { name: string }) {
  const slug = this.generateSlug(data.name);

  return this.prisma.guild.create({
    data: {
      ...data,
      slug,
    },
  });
}

async getGuildBySlug(slug: string) {
  return this.prisma.guild.findUnique({
    where: { slug },
  });
}

async searchGuilds(query: string) {
  return this.prisma.guild.findMany({
    where: {
      name: {
        contains: query,
        mode: 'insensitive',
      },
    },
  });
}