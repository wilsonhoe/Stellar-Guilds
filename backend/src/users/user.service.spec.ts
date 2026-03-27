import { UserService } from './user.service';
import { prismaMock } from '../../test/mocks/prisma.mock';

describe('UserService', () => {
  let userService: UserService;

  beforeEach(() => {
    userService = new UserService(prismaMock as any);
    jest.clearAllMocks();
  });

  describe('createUser', () => {
    it('should create a user successfully', async () => {
      const mockUser = {
        id: '1',
        email: 'test@example.com',
        name: 'Test User',
      };

      prismaMock.user.create.mockResolvedValue(mockUser as any);

      const result = await userService.createUser({
        email: 'test@example.com',
        name: 'Test User',
      });

      expect(result).toEqual(mockUser);

      expect(prismaMock.user.create).toHaveBeenCalledWith({
        data: {
          email: 'test@example.com',
          name: 'Test User',
        },
      });
    });
  });

  describe('getUserById', () => {
    it('should return user if found', async () => {
      const mockUser = {
        id: '1',
        email: 'test@example.com',
      };

      prismaMock.user.findUnique.mockResolvedValue(mockUser as any);

      const result = await userService.getUserById('1');

      expect(result).toEqual(mockUser);

      expect(prismaMock.user.findUnique).toHaveBeenCalledWith({
        where: { id: '1' },
      });
    });

    it('should return null if user not found', async () => {
      prismaMock.user.findUnique.mockResolvedValue(null);

      const result = await userService.getUserById('999');

      expect(result).toBeNull();
    });
  });

  describe('deleteUser', () => {
    it('should delete a user', async () => {
      const mockUser = { id: '1' };

      prismaMock.user.delete.mockResolvedValue(mockUser as any);

      const result = await userService.deleteUser('1');

      expect(result).toEqual(mockUser);

      expect(prismaMock.user.delete).toHaveBeenCalledWith({
        where: { id: '1' },
      });
    });
  });
});