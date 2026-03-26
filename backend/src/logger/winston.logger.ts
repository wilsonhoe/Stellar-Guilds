import * as winston from 'winston';
import { Logger as NestLogger } from '@nestjs/common';

export class WinstonLogger extends NestLogger {
  private winstonLogger: winston.Logger;

  constructor(context?: string) {
    super(context);
    this.winstonLogger = this.createWinstonLogger();
  }

  private createWinstonLogger(): winston.Logger {
    const logFormat = winston.format.combine(
      winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss.SSS' }),
      winston.format.errors({ stack: true }),
      winston.format.splat(),
      winston.format.json(),
      winston.format.printf(
        ({ timestamp, level, message, context, ...meta }) => {
          return JSON.stringify({
            timestamp,
            level: level.toUpperCase(),
            context: context || 'Application',
            message,
            ...(Object.keys(meta).length > 0 && { meta }),
          });
        },
      ),
    );

    return winston.createLogger({
      level: process.env.LOG_LEVEL || 'info',
      format: logFormat,
      defaultMeta: { service: 'stellar-guilds' },
      transports: [
        new winston.transports.Console({
          format: logFormat,
        }),
        // Optional: Add file transport for production
        ...(process.env.NODE_ENV === 'production'
          ? [
              new winston.transports.File({
                filename: 'logs/error.log',
                level: 'error',
                format: logFormat,
              }),
              new winston.transports.File({
                filename: 'logs/combined.log',
                format: logFormat,
              }),
            ]
          : []),
      ],
    });
  }

  log(message: string, context?: string) {
    this.winstonLogger.info(message, { context: context || this.context });
  }

  error(message: string, trace?: string, context?: string) {
    this.winstonLogger.error(message, {
      context: context || this.context,
      stack: trace,
    });
  }

  warn(message: string, context?: string) {
    this.winstonLogger.warn(message, { context: context || this.context });
  }

  debug(message: string, context?: string) {
    this.winstonLogger.debug(message, { context: context || this.context });
  }

  verbose(message: string, context?: string) {
    this.winstonLogger.debug(message, { context: context || this.context });
  }
}
