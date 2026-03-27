constructor(private prisma: PrismaClient) {}

async createUser(data) {
  return this.prisma.user.create({ data });
}

async getUserById(id: string) {
  return this.prisma.user.findUnique({
    where: { id },
  });
}

async deleteUser(id: string) {
  return this.prisma.user.delete({
    where: { id },
  });
}