import { GuildService } from './guild.service';
import { prismaMock } from '../../test/mocks/prisma.mock';

describe('GuildService', () => {
  let guildService: GuildService;

  beforeEach(() => {
    guildService = new GuildService(prismaMock as any);
    jest.clearAllMocks();
  });

  describe('createGuild', () => {
    it('should create a guild with generated slug', async () => {
      const input = {
        name: 'My Test Guild',
      };

      const mockGuild = {
        id: '1',
        name: 'My Test Guild',
        slug: 'my-test-guild',
      };

      prismaMock.guild.create.mockResolvedValue(mockGuild as any);

      const result = await guildService.createGuild(input);

      expect(result).toEqual(mockGuild);

      expect(prismaMock.guild.create).toHaveBeenCalledWith({
        data: expect.objectContaining({
          name: 'My Test Guild',
          slug: 'my-test-guild',
        }),
      });
    });

    it('should handle duplicate slug scenario', async () => {
      prismaMock.guild.create.mockRejectedValue(
        new Error('Unique constraint failed')
      );

      await expect(
        guildService.createGuild({ name: 'Duplicate Guild' })
      ).rejects.toThrow();
    });
  });

  describe('getGuildBySlug', () => {
    it('should return guild if found', async () => {
      const mockGuild = {
        id: '1',
        name: 'Guild',
        slug: 'guild',
      };

      prismaMock.guild.findUnique.mockResolvedValue(mockGuild as any);

      const result = await guildService.getGuildBySlug('guild');

      expect(result).toEqual(mockGuild);

      expect(prismaMock.guild.findUnique).toHaveBeenCalledWith({
        where: { slug: 'guild' },
      });
    });

    it('should return null if not found', async () => {
      prismaMock.guild.findUnique.mockResolvedValue(null);

      const result = await guildService.getGuildBySlug('unknown');

      expect(result).toBeNull();
    });
  });

  describe('searchGuilds', () => {
    it('should return list of guilds based on query', async () => {
      const mockGuilds = [
        { id: '1', name: 'Alpha Guild', slug: 'alpha-guild' },
      ];

      prismaMock.guild.findMany.mockResolvedValue(mockGuilds as any);

      const result = await guildService.searchGuilds('alpha');

      expect(result).toEqual(mockGuilds);

      expect(prismaMock.guild.findMany).toHaveBeenCalledWith({
        where: {
          name: {
            contains: 'alpha',
            mode: 'insensitive',
          },
        },
      });
    });
  });
});