import { validate } from 'class-validator';
import { plainToInstance } from 'class-transformer';
import { CreateGuildDto } from './create-guild.dto';

describe('CreateGuildDto', () => {
  it('should validate a valid guild creation', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'test-guild',
      description: 'A test guild',
    });

    const errors = await validate(dto);
    expect(errors).toHaveLength(0);
  });

  it('should transform slug to lowercase', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'TEST-GUILD',
    });

    expect(dto.slug).toBe('test-guild');
  });

  it('should trim whitespace from slug', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: '  test-guild  ',
    });

    expect(dto.slug).toBe('test-guild');
  });

  it('should transform slug to lowercase and validate', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'TEST-GUILD',
    });

    // slug should be transformed to lowercase
    expect(dto.slug).toBe('test-guild');

    const errors = await validate(dto);
    expect(errors).toHaveLength(0);
  });

  it('should reject slug with special characters', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'test@guild!',
    });

    const errors = await validate(dto);
    expect(errors.length).toBeGreaterThan(0);
    expect(errors[0].constraints?.matches).toContain('lowercase');
  });

  it('should reject slug with spaces', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'test guild',
    });

    const errors = await validate(dto);
    expect(errors.length).toBeGreaterThan(0);
  });

  it('should accept slug with numbers and hyphens', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'test-guild-123',
    });

    const errors = await validate(dto);
    expect(errors).toHaveLength(0);
  });

  it('should reject empty slug', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: '',
    });

    const errors = await validate(dto);
    expect(errors.length).toBeGreaterThan(0);
  });

  it('should reject name exceeding max length', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'A'.repeat(101),
      slug: 'test-guild',
    });

    const errors = await validate(dto);
    expect(errors.length).toBeGreaterThan(0);
  });

  it('should reject slug exceeding max length', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'a'.repeat(101),
    });

    const errors = await validate(dto);
    expect(errors.length).toBeGreaterThan(0);
  });

  it('should allow optional slug', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
    });

    const errors = await validate(dto);
    expect(errors).toHaveLength(0);
  });

  it('should allow optional description', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'test-guild',
    });

    const errors = await validate(dto);
    expect(errors).toHaveLength(0);
  });

  it('should allow optional settings', async () => {
    const dto = plainToInstance(CreateGuildDto, {
      name: 'Test Guild',
      slug: 'test-guild',
      settings: { discoverable: true },
    });

    const errors = await validate(dto);
    expect(errors).toHaveLength(0);
  });
});
