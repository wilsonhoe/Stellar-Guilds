import { mockDeep, DeepMockProxy } from 'jest-mock-extended';
import { PrismaClient } from '@prisma/client';

export const prismaMock: DeepMockProxy<PrismaClient> =
  mockDeep<PrismaClient>();