import { Prisma, PrismaClient } from '@prisma/client';

// Models that support soft delete
const softDeleteModels = ['User', 'Guild', 'Bounty'] as const;
type SoftDeleteModel = (typeof softDeleteModels)[number];

/**
 * Check if a model supports soft delete
 */
function isSoftDeleteModel(model: string): model is SoftDeleteModel {
  return softDeleteModels.includes(model as SoftDeleteModel);
}

/**
 * Prisma Client Extension for Soft Delete
 *
 * Intercepts delete queries and converts them to update queries setting deletedAt timestamp.
 * Also intercepts findMany and findUnique to filter out soft-deleted records by default.
 */
export function createSoftDeleteExtension(prisma: PrismaClient) {
  return prisma.$extends({
    model: {
      $allModels: {
        /**
         * Soft delete - sets deletedAt instead of actually deleting
         */
        async softDelete<T>(this: T, where: Record<string, any>): Promise<T> {
          const context = Prisma.getExtensionContext(this) as any;
          const model = context.$name as string;

          if (!isSoftDeleteModel(model)) {
            // For models without soft delete, use actual delete
            return context.delete({ where });
          }

          return context.update({
            where,
            data: { deletedAt: new Date() },
          });
        },

        /**
         * Restore soft-deleted record
         */
        async restore<T>(this: T, where: Record<string, any>): Promise<T> {
          const context = Prisma.getExtensionContext(this) as any;
          const model = context.$name as string;

          if (!isSoftDeleteModel(model)) {
            throw new Error(`Model ${model} does not support soft delete`);
          }

          return context.update({
            where,
            data: { deletedAt: null },
          });
        },

        /**
         * Find including soft-deleted records
         */
        async findManyWithDeleted<T>(
          this: T,
          args: Record<string, any> = {},
        ): Promise<T[]> {
          const context = Prisma.getExtensionContext(this) as any;
          return context.findMany(args);
        },

        /**
         * Find unique including soft-deleted records
         */
        async findUniqueWithDeleted<T>(
          this: T,
          args: Record<string, any>,
        ): Promise<T | null> {
          const context = Prisma.getExtensionContext(this) as any;
          return context.findUnique(args);
        },
      },
    },
    query: {
      $allModels: {
        async findMany({ model, args, query }) {
          if (isSoftDeleteModel(model)) {
            const where = (args.where || {}) as Record<string, any>;
            // Only filter deletedAt if not explicitly queried
            if (!('deletedAt' in where)) {
              (args as any).where = {
                ...where,
                deletedAt: null,
              };
            }
          }
          return query(args);
        },

        async findUnique({ model, args, query }) {
          if (isSoftDeleteModel(model)) {
            args.where = {
              ...args.where,
              deletedAt: null,
            } as any;
          }
          return query(args);
        },

        async findFirst({ model, args, query }) {
          if (isSoftDeleteModel(model)) {
            const where = (args.where || {}) as Record<string, any>;
            if (!('deletedAt' in where)) {
              (args as any).where = {
                ...where,
                deletedAt: null,
              };
            }
          }
          return query(args);
        },

        async count({ model, args, query }) {
          if (isSoftDeleteModel(model) && args.where) {
            const where = args.where as Record<string, any>;
            if (!('deletedAt' in where)) {
              (args as any).where = {
                ...where,
                deletedAt: null,
              };
            }
          }
          return query(args);
        },

        async delete({ model, args, query }) {
          if (isSoftDeleteModel(model)) {
            // Convert delete to soft delete (update deletedAt)
            return (prisma as any)[model.toLowerCase()].update({
              where: args.where,
              data: { deletedAt: new Date() },
            });
          }
          return query(args);
        },

        async deleteMany({ model, args, query }) {
          if (isSoftDeleteModel(model)) {
            // Convert deleteMany to soft delete many (update deletedAt)
            return (prisma as any)[model.toLowerCase()].updateMany({
              where: args.where,
              data: { deletedAt: new Date() },
            });
          }
          return query(args);
        },
      },
    },
  });
}
