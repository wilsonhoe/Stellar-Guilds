import { Controller, Get } from '@nestjs/common';
import {
  HealthCheck,
  HealthCheckService,
  PrismaHealthIndicator,
} from '@nestjs/terminus';
import { PrismaService } from '../prisma/prisma.service';

@Controller('health')
export class HealthController {
  constructor(
    private health: HealthCheckService,
    private prismaHealth: PrismaHealthIndicator,
    private prismaService: PrismaService,
  ) {}

  @Get()
  @HealthCheck()
  async check() {
    return this.health.check([
      () =>
        this.prismaHealth.pingCheck('database', this.prismaService['prisma']),
    ]);
  }

  @Get('uptime')
  getUptime() {
    const uptime = process.uptime();
    const hours = Math.floor(uptime / 3600);
    const minutes = Math.floor((uptime % 3600) / 60);
    const seconds = Math.floor(uptime % 60);

    return {
      status: 'ok',
      uptime: {
        seconds: Math.floor(uptime),
        formatted: `${hours}h ${minutes}m ${seconds}s`,
      },
      timestamp: new Date().toISOString(),
    };
  }

  @Get('status')
  async getStatus() {
    const uptime = process.uptime();
    const hours = Math.floor(uptime / 3600);
    const minutes = Math.floor((uptime % 3600) / 60);
    const seconds = Math.floor(uptime % 60);

    try {
      // Test database connection
      await this.prismaService.$queryRaw`SELECT 1`;
      const dbStatus = 'connected';

      return {
        status: 'healthy',
        uptime: {
          seconds: Math.floor(uptime),
          formatted: `${hours}h ${minutes}m ${seconds}s`,
        },
        database: {
          status: dbStatus,
          connection: 'healthy',
        },
        timestamp: new Date().toISOString(),
      };
    } catch (error: any) {
      return {
        status: 'unhealthy',
        uptime: {
          seconds: Math.floor(uptime),
          formatted: `${hours}h ${minutes}m ${seconds}s`,
        },
        database: {
          status: 'disconnected',
          connection: 'unhealthy',
          error: error?.message || 'Unknown database error',
        },
        timestamp: new Date().toISOString(),
      };
    }
  }
}
