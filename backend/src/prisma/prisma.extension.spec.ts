import { PrismaClient } from '@prisma/client';
import { createSoftDeleteExtension } from './prisma.extension';

describe('Prisma Soft Delete Extension', () => {
  let basePrisma: PrismaClient;
  let prisma: ReturnType<typeof createSoftDeleteExtension>;

  beforeEach(() => {
    basePrisma = new PrismaClient();
    prisma = createSoftDeleteExtension(basePrisma);
  });

  afterEach(async () => {
    await basePrisma.$disconnect();
  });

  describe('Model extension methods', () => {
    it('should have softDelete method on User model', () => {
      expect(typeof prisma.user.softDelete).toBe('function');
    });

    it('should have restore method on User model', () => {
      expect(typeof prisma.user.restore).toBe('function');
    });

    it('should have findManyWithDeleted method on User model', () => {
      expect(typeof prisma.user.findManyWithDeleted).toBe('function');
    });

    it('should have findUniqueWithDeleted method on User model', () => {
      expect(typeof prisma.user.findUniqueWithDeleted).toBe('function');
    });

    it('should have softDelete method on Guild model', () => {
      expect(typeof prisma.guild.softDelete).toBe('function');
    });

    it('should have softDelete method on Bounty model', () => {
      expect(typeof prisma.bounty.softDelete).toBe('function');
    });
  });

  describe('Query interception', () => {
    describe('findMany', () => {
      it('should add deletedAt: null filter for soft delete models', async () => {
        const findManySpy = jest
          .spyOn(basePrisma.user, 'findMany')
          .mockResolvedValue([]);

        await prisma.user.findMany({ where: { username: 'test' } });

        expect(findManySpy).toHaveBeenCalledWith({
          where: {
            username: 'test',
            deletedAt: null,
          },
        });

        findManySpy.mockRestore();
      });

      it('should not add deletedAt filter if already specified', async () => {
        const findManySpy = jest
          .spyOn(basePrisma.user, 'findMany')
          .mockResolvedValue([]);

        await prisma.user.findMany({
          where: { deletedAt: new Date() } as any,
        });

        expect(findManySpy).toHaveBeenCalledWith({
          where: {
            deletedAt: expect.any(Date),
          },
        });

        findManySpy.mockRestore();
      });

      it('should not add deletedAt filter for non-soft-delete models', async () => {
        const findManySpy = jest
          .spyOn(basePrisma.role, 'findMany')
          .mockResolvedValue([]);

        await prisma.role.findMany({ where: { name: 'test' } });

        expect(findManySpy).toHaveBeenCalledWith({
          where: {
            name: 'test',
          },
        });

        findManySpy.mockRestore();
      });
    });

    describe('findUnique', () => {
      it('should add deletedAt: null filter for soft delete models', async () => {
        const findUniqueSpy = jest
          .spyOn(basePrisma.user, 'findUnique')
          .mockResolvedValue(null);

        await prisma.user.findUnique({ where: { id: 'user-1' } });

        expect(findUniqueSpy).toHaveBeenCalledWith({
          where: {
            id: 'user-1',
            deletedAt: null,
          },
        });

        findUniqueSpy.mockRestore();
      });
    });

    describe('findFirst', () => {
      it('should add deletedAt: null filter for soft delete models', async () => {
        const findFirstSpy = jest
          .spyOn(basePrisma.user, 'findFirst')
          .mockResolvedValue(null);

        await prisma.user.findFirst({ where: { email: 'test@example.com' } });

        expect(findFirstSpy).toHaveBeenCalledWith({
          where: {
            email: 'test@example.com',
            deletedAt: null,
          },
        });

        findFirstSpy.mockRestore();
      });
    });

    describe('count', () => {
      it('should add deletedAt: null filter for soft delete models', async () => {
        const countSpy = jest
          .spyOn(basePrisma.user, 'count')
          .mockResolvedValue(0);

        await prisma.user.count({ where: { role: 'USER' } });

        expect(countSpy).toHaveBeenCalledWith({
          where: {
            role: 'USER',
            deletedAt: null,
          },
        });

        countSpy.mockRestore();
      });
    });

    describe('delete', () => {
      it('should convert delete to soft delete for soft delete models', async () => {
        const updateSpy = jest
          .spyOn(basePrisma.user, 'update')
          .mockResolvedValue({} as any);

        await prisma.user.delete({ where: { id: 'user-1' } });

        expect(updateSpy).toHaveBeenCalledWith({
          where: { id: 'user-1' },
          data: { deletedAt: expect.any(Date) },
        });

        updateSpy.mockRestore();
      });

      it('should perform hard delete for non-soft-delete models', async () => {
        const deleteSpy = jest
          .spyOn(basePrisma.role, 'delete')
          .mockResolvedValue({} as any);

        await prisma.role.delete({ where: { id: 'role-1' } });

        expect(deleteSpy).toHaveBeenCalledWith({
          where: { id: 'role-1' },
        });

        deleteSpy.mockRestore();
      });
    });

    describe('deleteMany', () => {
      it('should convert deleteMany to soft delete many for soft delete models', async () => {
        const updateManySpy = jest
          .spyOn(basePrisma.user, 'updateMany')
          .mockResolvedValue({ count: 5 } as any);

        await prisma.user.deleteMany({ where: { role: 'USER' } });

        expect(updateManySpy).toHaveBeenCalledWith({
          where: { role: 'USER' },
          data: { deletedAt: expect.any(Date) },
        });

        updateManySpy.mockRestore();
      });

      it('should perform hard deleteMany for non-soft-delete models', async () => {
        const deleteManySpy = jest
          .spyOn(basePrisma.role, 'deleteMany')
          .mockResolvedValue({ count: 3 } as any);

        await prisma.role.deleteMany({ where: { name: { contains: 'test' } } });

        expect(deleteManySpy).toHaveBeenCalledWith({
          where: { name: { contains: 'test' } },
        });

        deleteManySpy.mockRestore();
      });
    });
  });

  describe('findManyWithDeleted', () => {
    it('should not filter deletedAt when using findManyWithDeleted', async () => {
      const findManySpy = jest
        .spyOn(basePrisma.user, 'findMany')
        .mockResolvedValue([]);

      await prisma.user.findManyWithDeleted({ where: { role: 'USER' } });

      expect(findManySpy).toHaveBeenCalledWith({
        where: { role: 'USER' },
      });

      findManySpy.mockRestore();
    });
  });

  describe('findUniqueWithDeleted', () => {
    it('should not filter deletedAt when using findUniqueWithDeleted', async () => {
      const findUniqueSpy = jest
        .spyOn(basePrisma.user, 'findUnique')
        .mockResolvedValue(null);

      await prisma.user.findUniqueWithDeleted({ where: { id: 'user-1' } });

      expect(findUniqueSpy).toHaveBeenCalledWith({
        where: { id: 'user-1' },
      });

      findUniqueSpy.mockRestore();
    });
  });
});
