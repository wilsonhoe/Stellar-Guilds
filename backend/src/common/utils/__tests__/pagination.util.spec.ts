import { PaginationUtil } from '../pagination.util';

describe('PaginationUtil', () => {
  describe('getPagination', () => {
    it('should return correct skip and take', () => {
      const result = PaginationUtil.getPagination({ page: 2, limit: 10 });

      expect(result.skip).toBe(10);
      expect(result.take).toBe(10);
      expect(result.page).toBe(2);
      expect(result.limit).toBe(10);
    });

    it('should fallback to defaults', () => {
      const result = PaginationUtil.getPagination({});

      expect(result.page).toBe(1);
      expect(result.limit).toBe(10);
    });

    it('should not exceed max limit', () => {
      const result = PaginationUtil.getPagination({ limit: 500 });

      expect(result.limit).toBe(100);
    });
  });

  describe('buildMeta', () => {
    it('should calculate metadata correctly', () => {
      const meta = PaginationUtil.buildMeta(100, 2, 10);

      expect(meta.total).toBe(100);
      expect(meta.totalPages).toBe(10);
      expect(meta.hasNextPage).toBe(true);
      expect(meta.hasPreviousPage).toBe(true);
    });

    it('should detect last page', () => {
      const meta = PaginationUtil.buildMeta(20, 2, 10);

      expect(meta.hasNextPage).toBe(false);
    });
  });

  describe('buildResult', () => {
    it('should return data with meta', () => {
      const data = [{ id: 1 }, { id: 2 }];

      const result = PaginationUtil.buildResult(data, 20, 1, 10);

      expect(result.data).toEqual(data);
      expect(result.meta.total).toBe(20);
    });
  });
});
