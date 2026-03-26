import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, HttpStatus } from '@nestjs/common';
import request from 'supertest';
import { JwtService } from '@nestjs/jwt';
import { AppModule } from '../app.module';
import { UserRole } from '../user/dto/user.dto';

/**
 * Integration tests for Role-Based Access Control (RBAC)
 *
 * These tests demonstrate how to test protected endpoints
 * with different user roles.
 */
describe('Role-Based Access Control (RBAC) - Integration Tests', () => {
  let app: INestApplication;
  let jwtService: JwtService;

  // Test tokens for different roles
  const testTokens: Record<string, string> = {};

  beforeAll(async () => {
    const moduleFixture: TestingModule = await Test.createTestingModule({
      imports: [AppModule],
    }).compile();

    app = moduleFixture.createNestApplication();
    await app.init();

    jwtService = moduleFixture.get<JwtService>(JwtService);

    // Generate test tokens for each role
    const roles = [
      UserRole.USER,
      UserRole.MODERATOR,
      UserRole.ADMIN,
      UserRole.OWNER,
    ];

    for (const role of roles) {
      const token = jwtService.sign({
        sub: `test-user-${role}`,
        email: `test.${role.toLowerCase()}@example.com`,
        role: role,
      });
      testTokens[role] = token;
    }
  });

  afterAll(async () => {
    await app.close();
  });

  describe('Public Endpoints', () => {
    it('should allow access without authentication', async () => {
      return request(app.getHttpServer())
        .get('/examples/public')
        .expect(HttpStatus.OK);
    });
  });

  describe('Authenticated Endpoints (Any Role)', () => {
    it('should allow access with valid USER token', async () => {
      return request(app.getHttpServer())
        .get('/examples/authenticated')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.OK);
    });

    it('should allow access with valid ADMIN token', async () => {
      return request(app.getHttpServer())
        .get('/examples/authenticated')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .expect(HttpStatus.OK);
    });

    it('should deny access without token', async () => {
      return request(app.getHttpServer())
        .get('/examples/authenticated')
        .expect(HttpStatus.UNAUTHORIZED);
    });

    it('should deny access with invalid token', async () => {
      return request(app.getHttpServer())
        .get('/examples/authenticated')
        .set('Authorization', 'Bearer invalid-token')
        .expect(HttpStatus.UNAUTHORIZED);
    });
  });

  describe('Moderator-Only Endpoints', () => {
    it('should allow access with MODERATOR token', async () => {
      return request(app.getHttpServer())
        .get('/examples/moderator')
        .set('Authorization', `Bearer ${testTokens[UserRole.MODERATOR]}`)
        .expect(HttpStatus.OK);
    });

    it('should allow access with ADMIN token', async () => {
      return request(app.getHttpServer())
        .get('/examples/moderator')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .expect(HttpStatus.OK);
    });

    it('should allow access with OWNER token', async () => {
      return request(app.getHttpServer())
        .get('/examples/moderator')
        .set('Authorization', `Bearer ${testTokens[UserRole.OWNER]}`)
        .expect(HttpStatus.OK);
    });

    it('should deny access with USER token (insufficient role)', async () => {
      const response = await request(app.getHttpServer())
        .get('/examples/moderator')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.FORBIDDEN);

      expect(response.body.message).toContain('does not have access');
    });

    it('should deny access without token', async () => {
      return request(app.getHttpServer())
        .get('/examples/moderator')
        .expect(HttpStatus.UNAUTHORIZED);
    });
  });

  describe('Admin-Only Endpoints', () => {
    it('should allow access with ADMIN token', async () => {
      return request(app.getHttpServer())
        .get('/examples/admin')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .expect(HttpStatus.OK);
    });

    it('should allow access with OWNER token', async () => {
      return request(app.getHttpServer())
        .get('/examples/admin')
        .set('Authorization', `Bearer ${testTokens[UserRole.OWNER]}`)
        .expect(HttpStatus.OK);
    });

    it('should deny access with MODERATOR token', async () => {
      const response = await request(app.getHttpServer())
        .get('/examples/admin')
        .set('Authorization', `Bearer ${testTokens[UserRole.MODERATOR]}`)
        .expect(HttpStatus.FORBIDDEN);

      expect(response.body.message).toContain('does not have access');
    });

    it('should deny access with USER token', async () => {
      const response = await request(app.getHttpServer())
        .get('/examples/admin')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.FORBIDDEN);

      expect(response.body.message).toContain('does not have access');
    });
  });

  describe('Owner-Only Endpoints', () => {
    it('should allow access with OWNER token', async () => {
      return request(app.getHttpServer())
        .get('/examples/owner')
        .set('Authorization', `Bearer ${testTokens[UserRole.OWNER]}`)
        .expect(HttpStatus.OK);
    });

    it('should deny access with ADMIN token', async () => {
      const response = await request(app.getHttpServer())
        .get('/examples/owner')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .expect(HttpStatus.FORBIDDEN);

      expect(response.body.message).toContain('does not have access');
    });

    it('should deny access with all other roles', async () => {
      // Test MODERATOR
      await request(app.getHttpServer())
        .get('/examples/owner')
        .set('Authorization', `Bearer ${testTokens[UserRole.MODERATOR]}`)
        .expect(HttpStatus.FORBIDDEN);

      // Test USER
      await request(app.getHttpServer())
        .get('/examples/owner')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.FORBIDDEN);
    });
  });

  describe('POST Endpoints with RBAC', () => {
    const testData = { name: 'Test Resource', value: 123 };

    it('should allow ADMIN to create resources', async () => {
      return request(app.getHttpServer())
        .post('/examples/admin/create')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .send(testData)
        .expect(HttpStatus.CREATED);
    });

    it('should allow OWNER to create resources', async () => {
      return request(app.getHttpServer())
        .post('/examples/admin/create')
        .set('Authorization', `Bearer ${testTokens[UserRole.OWNER]}`)
        .send(testData)
        .expect(HttpStatus.CREATED);
    });

    it('should deny MODERATOR from creating resources', async () => {
      return request(app.getHttpServer())
        .post('/examples/admin/create')
        .set('Authorization', `Bearer ${testTokens[UserRole.MODERATOR]}`)
        .send(testData)
        .expect(HttpStatus.FORBIDDEN);
    });
  });

  describe('DELETE Endpoints with RBAC', () => {
    it('should allow ADMIN to delete resources', async () => {
      return request(app.getHttpServer())
        .delete('/examples/admin/delete/test-id-123')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .expect(HttpStatus.OK);
    });

    it('should deny USER from deleting resources', async () => {
      return request(app.getHttpServer())
        .delete('/examples/admin/delete/test-id-123')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.FORBIDDEN);
    });
  });

  describe('Multi-Role Endpoints', () => {
    it('should allow MODERATOR access to content management', async () => {
      return request(app.getHttpServer())
        .get('/examples/content/manage')
        .set('Authorization', `Bearer ${testTokens[UserRole.MODERATOR]}`)
        .expect(HttpStatus.OK);
    });

    it('should allow ADMIN access to content management', async () => {
      return request(app.getHttpServer())
        .get('/examples/content/manage')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .expect(HttpStatus.OK);
    });

    it('should deny USER access to content management', async () => {
      return request(app.getHttpServer())
        .get('/examples/content/manage')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.FORBIDDEN);
    });
  });

  describe('User Management Endpoints (Real Examples)', () => {
    it('should allow ADMIN to get users by role', async () => {
      return request(app.getHttpServer())
        .get('/examples/users/by-role/USER')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .expect(HttpStatus.OK);
    });

    it('should allow MODERATOR to get users by role', async () => {
      return request(app.getHttpServer())
        .get('/examples/users/by-role/USER')
        .set('Authorization', `Bearer ${testTokens[UserRole.MODERATOR]}`)
        .expect(HttpStatus.OK);
    });

    it('should deny USER from getting users by role', async () => {
      return request(app.getHttpServer())
        .get('/examples/users/by-role/USER')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.FORBIDDEN);
    });

    it('should allow only ADMIN to assign roles', async () => {
      return request(app.getHttpServer())
        .post('/examples/users/test-user-id/assign-role')
        .set('Authorization', `Bearer ${testTokens[UserRole.ADMIN]}`)
        .send({ role: UserRole.MODERATOR })
        .expect(HttpStatus.CREATED);
    });

    it('should deny MODERATOR from assigning roles', async () => {
      return request(app.getHttpServer())
        .post('/examples/users/test-user-id/assign-role')
        .set('Authorization', `Bearer ${testTokens[UserRole.MODERATOR]}`)
        .send({ role: UserRole.USER })
        .expect(HttpStatus.FORBIDDEN);
    });
  });

  describe('Error Messages', () => {
    it('should return 401 with clear message for missing token', async () => {
      const response = await request(app.getHttpServer())
        .get('/examples/admin')
        .expect(HttpStatus.UNAUTHORIZED);

      expect(response.body.message).toBe('Unauthorized');
    });

    it('should return 403 with clear message for insufficient role', async () => {
      const response = await request(app.getHttpServer())
        .get('/examples/admin')
        .set('Authorization', `Bearer ${testTokens[UserRole.USER]}`)
        .expect(HttpStatus.FORBIDDEN);

      expect(response.body.message).toContain('USER');
      expect(response.body.message).toContain('does not have access');
    });
  });
});

/**
 * UNIT TEST EXAMPLES
 *
 * For unit testing specific guards:
 */
describe('RoleGuard - Unit Tests', () => {
  it('can be tested independently with mock ExecutionContext', () => {
    // Example structure for unit testing the guard itself
    // See backend/src/auth/guards/role.guard.spec.ts for implementation
  });
});

/**
 * TESTING TIPS:
 *
 * 1. Use realistic test data for each role
 * 2. Test both positive (allowed) and negative (denied) cases
 * 3. Verify error messages are clear and helpful
 * 4. Test edge cases (expired tokens, malformed tokens)
 * 5. Use beforeEach to reset state between tests
 * 6. Mock external dependencies (database, services)
 * 7. Test the guard logic separately from controllers
 *
 * RUNNING TESTS:
 *
 * npm run test -- rbac.integration.spec.ts
 * npm run test:e2e -- rbac.integration.spec.ts
 */
