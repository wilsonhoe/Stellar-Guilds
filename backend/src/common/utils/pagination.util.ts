import {
  PaginationQuery,
  PaginationMeta,
  PaginatedResult,
} from './pagination.types';

const DEFAULT_PAGE = 1;
const DEFAULT_LIMIT = 10;
const MAX_LIMIT = 100;

export class PaginationUtil {
  /**
   * Converts page/limit into Prisma skip/take
   */
  static getPagination(query: PaginationQuery) {
    const page = Math.max(query.page || DEFAULT_PAGE, 1);
    const limit = Math.min(query.limit || DEFAULT_LIMIT, MAX_LIMIT);

    const skip = (page - 1) * limit;
    const take = limit;

    return { skip, take, page, limit };
  }

  /**
   * Builds pagination metadata
   */
  static buildMeta(total: number, page: number, limit: number): PaginationMeta {
    const totalPages = Math.ceil(total / limit);

    return {
      total,
      page,
      limit,
      totalPages,
      hasNextPage: page < totalPages,
      hasPreviousPage: page > 1,
    };
  }

  /**
   * Wraps Prisma result into a paginated response
   */
  static buildResult<T>(
    data: T[],
    total: number,
    page: number,
    limit: number,
  ): PaginatedResult<T> {
    return {
      data,
      meta: this.buildMeta(total, page, limit),
    };
  }
}
