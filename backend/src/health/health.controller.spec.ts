import { Test, TestingModule } from '@nestjs/testing';
import { HealthController } from './health.controller';
import { HealthCheckService } from '@nestjs/terminus';
import { PrismaHealthIndicator } from '@nestjs/terminus';
import { PrismaService } from '../prisma/prisma.service';

describe('HealthController', () => {
  let controller: HealthController;
  let healthCheckService: HealthCheckService;
  let prismaHealthIndicator: PrismaHealthIndicator;
  let prismaService: PrismaService;

  beforeEach(async () => {
    const mockHealthCheckService = {
      check: jest.fn(),
    };

    const mockPrismaHealthIndicator = {
      pingCheck: jest.fn(),
    };

    const mockPrismaService = {
      prisma: {
        $queryRaw: jest.fn(),
      },
    };

    const module: TestingModule = await Test.createTestingModule({
      controllers: [HealthController],
      providers: [
        {
          provide: HealthCheckService,
          useValue: mockHealthCheckService,
        },
        {
          provide: PrismaHealthIndicator,
          useValue: mockPrismaHealthIndicator,
        },
        {
          provide: PrismaService,
          useValue: mockPrismaService,
        },
      ],
    }).compile();

    controller = module.get<HealthController>(HealthController);
    healthCheckService = module.get<HealthCheckService>(HealthCheckService);
    prismaHealthIndicator = module.get<PrismaHealthIndicator>(
      PrismaHealthIndicator,
    );
    prismaService = module.get<PrismaService>(PrismaService);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });

  it('should return uptime information', () => {
    const result = controller.getUptime();

    expect(result).toHaveProperty('status', 'ok');
    expect(result).toHaveProperty('uptime');
    expect(result).toHaveProperty('timestamp');
    expect(result.uptime).toHaveProperty('seconds');
    expect(result.uptime).toHaveProperty('formatted');
  });

  it('should return healthy status when database is connected', async () => {
    const mockQueryRaw = jest.fn().mockResolvedValue('OK');
    Object.defineProperty(prismaService, '$queryRaw', {
      get: () => mockQueryRaw,
      configurable: true,
    });

    const result = await controller.getStatus();

    expect(result).toHaveProperty('status', 'healthy');
    expect(result).toHaveProperty('database');
    expect(result.database).toHaveProperty('status', 'connected');
    expect(result.database).toHaveProperty('connection', 'healthy');
  });

  it('should return unhealthy status when database fails', async () => {
    const mockQueryRaw = jest
      .fn()
      .mockRejectedValue(new Error('Connection failed'));
    Object.defineProperty(prismaService, '$queryRaw', {
      get: () => mockQueryRaw,
      configurable: true,
    });

    const result = await controller.getStatus();

    expect(result).toHaveProperty('status', 'unhealthy');
    expect(result).toHaveProperty('database');
    expect(result.database).toHaveProperty('status', 'disconnected');
    expect(result.database).toHaveProperty('connection', 'unhealthy');
    expect(result.database).toHaveProperty('error', 'Connection failed');
  });
});
