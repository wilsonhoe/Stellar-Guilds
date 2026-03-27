# Role-Based Access Control (RBAC) Guide

## Overview

Stellar-Guilds implements a complete Role-Based Access Control (RBAC) system using custom decorators and guards that integrate seamlessly with the existing JWT authentication system.

## Architecture

### Components

1. **@Roles() Decorator** - `backend/src/auth/decorators/roles.decorator.ts`
2. **RoleGuard** - `backend/src/auth/guards/role.guard.ts`
3. **UserRole Enum** - `backend/src/user/dto/user.dto.ts`
4. **JwtAuthGuard** - `backend/src/auth/guards/jwt-auth.guard.ts`

### User Roles

The system defines four role levels:

```typescript
export enum UserRole {
  USER = 'USER', // Basic user
  MODERATOR = 'MODERATOR', // Content moderator
  ADMIN = 'ADMIN', // Administrator
  OWNER = 'OWNER', // System owner
}
```

## How It Works

### 1. The @Roles() Decorator

The `@Roles()` decorator attaches metadata to route handlers, specifying which roles have access:

```typescript
import { Roles } from '../auth/decorators/roles.decorator';
import { UserRole } from '../user/dto/user.dto';

@Roles(UserRole.ADMIN, UserRole.MODERATOR)
@Get('admin/users')
async getAllUsers() {
  // Only ADMIN and MODERATOR can access
}
```

### 2. The RoleGuard

The `RoleGuard` intercepts requests and validates user roles:

```typescript
@Injectable()
export class RoleGuard implements CanActivate {
  constructor(private reflector: Reflector) {}

  canActivate(context: ExecutionContext): boolean {
    // Get required roles from metadata
    const requiredRoles = this.reflector.get<UserRole[]>(
      'roles',
      context.getHandler(),
    );

    if (!requiredRoles) {
      return true; // No role requirement
    }

    // Get user from request (set by JwtAuthGuard)
    const request = context.switchToHttp().getRequest();
    const user = request.user;

    if (!user) {
      throw new ForbiddenException('User not authenticated');
    }

    // Check if user has required role
    const hasRole = requiredRoles.includes(user.role);
    if (!hasRole) {
      throw new ForbiddenException(
        `User role "${user.role}" does not have access to this resource`,
      );
    }

    return true;
  }
}
```

## Usage

### Basic Usage

Combine `@UseGuards(JwtAuthGuard, RoleGuard)` with `@Roles()`:

```typescript
import { Controller, Get, UseGuards } from '@nestjs/common';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RoleGuard } from '../auth/guards/role.guard';
import { Roles } from '../auth/decorators/roles.decorator';
import { UserRole } from '../user/dto/user.dto';

@Controller('admin')
export class AdminController {
  // Only ADMIN users can access
  @Get('dashboard')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN)
  async getDashboard() {
    return { message: 'Admin dashboard data' };
  }

  // ADMIN or OWNER can access
  @Get('reports')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN, UserRole.OWNER)
  async getReports() {
    return { message: 'System reports' };
  }
}
```

### Real-World Examples from the Codebase

#### User Management Endpoints

```typescript
// backend/src/user/user.controller.ts

// Get users by role - ADMIN or MODERATOR only
@Get('role/:role')
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN, UserRole.MODERATOR)
async getUsersByRole(@Param('role') role: UserRole) {
  // Implementation
}

// Assign role to user - ADMIN only
@Patch(':userId/role')
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)
async assignRole(@Param('userId') userId: string, @Body() assignRoleDto: AssignRoleDto) {
  // Implementation
}

// Reactivate user - ADMIN only
@Post(':userId/reactivate')
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)
async reactivateUser(@Param('userId') userId: string) {
  // Implementation
}
```

## Integration with JWT Authentication

The RBAC system works in tandem with JWT authentication:

1. **JwtAuthGuard** first authenticates the user and decodes the JWT token
2. The decoded user (including their role) is attached to `request.user`
3. **RoleGuard** then checks if the user's role matches the required roles
4. Access is granted or denied based on role matching

### Authentication Flow

```
Request → JwtAuthGuard → Decode JWT → Attach user to request
             ↓
        RoleGuard → Check user.role against @Roles() metadata
             ↓
        Grant/Deny access
```

## Module Configuration

The guards are registered in the `AuthModule`:

```typescript
@Module({
  imports: [
    ConfigModule,
    PrismaModule,
    PassportModule,
    JwtModule.registerAsync({
      // JWT configuration
    }),
  ],
  controllers: [AuthController],
  providers: [AuthService, JwtStrategy, JwtAuthGuard, RoleGuard],
  exports: [AuthService, JwtStrategy, JwtAuthGuard, RoleGuard, PassportModule],
})
export class AuthModule {}
```

## Best Practices

### 1. Always Use JwtAuthGuard First

When protecting routes, always apply `JwtAuthGuard` before `RoleGuard`:

```typescript
✅ Correct:
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)

❌ Incorrect:
@UseGuards(RoleGuard, JwtAuthGuard)
@Roles(UserRole.ADMIN)
```

### 2. Be Specific with Role Requirements

Only require the minimum roles necessary:

```typescript
✅ For admin-only features:
@Roles(UserRole.ADMIN)

✅ For admin and moderators:
@Roles(UserRole.ADMIN, UserRole.MODERATOR)

❌ Too permissive:
@Roles(UserRole.USER, UserRole.MODERATOR, UserRole.ADMIN, UserRole.OWNER)
```

### 3. Document Role Requirements

Add comments explaining why specific roles are required:

```typescript
/**
 * Delete user account - ADMIN only
 * Sensitive operation requiring elevated privileges
 */
@Delete(':userId')
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)
async deleteUser(@Param('userId') userId: string) {
  // Implementation
}
```

### 4. Use Hierarchical Access

Structure your endpoints logically:

```typescript
// Public endpoint - no guard needed
@Get('public-info')
async getPublicInfo() { ... }

// Authenticated users only
@Get('profile')
@UseGuards(JwtAuthGuard)
async getProfile() { ... }

// Admin only
@Get('admin/stats')
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)
async getAdminStats() { ... }
```

## Testing RBAC Protected Endpoints

### Example Test Cases

```typescript
describe('RBAC Protected Endpoint', () => {
  it('should allow access with correct role', async () => {
    const adminToken = generateTestToken({ role: UserRole.ADMIN });

    return request(app.getHttpServer())
      .get('/admin/dashboard')
      .set('Authorization', `Bearer ${adminToken}`)
      .expect(200);
  });

  it('should deny access with insufficient role', async () => {
    const userToken = generateTestToken({ role: UserRole.USER });

    return request(app.getHttpServer())
      .get('/admin/dashboard')
      .set('Authorization', `Bearer ${userToken}`)
      .expect(403);
  });

  it('should deny access without authentication', async () => {
    return request(app.getHttpServer()).get('/admin/dashboard').expect(401);
  });
});
```

## Error Responses

### Unauthorized (401)

Returned when no JWT token is provided or token is invalid:

```json
{
  "statusCode": 401,
  "message": "Unauthorized"
}
```

### Forbidden (403)

Returned when user doesn't have required role:

```json
{
  "statusCode": 403,
  "message": "User role \"USER\" does not have access to this resource"
}
```

## Extending the System

### Adding New Roles

1. Update the `UserRole` enum:

```typescript
export enum UserRole {
  USER = 'USER',
  MODERATOR = 'MODERATOR',
  ADMIN = 'ADMIN',
  OWNER = 'OWNER',
  CONTRIBUTOR = 'CONTRIBUTOR', // New role
}
```

2. Update database schema if needed
3. Use the new role in `@Roles()` decorators

### Custom Guards for Specific Resources

For more granular control (e.g., guild-specific permissions), create specialized guards:

```typescript
// Example: GuildRoleGuard for guild-specific access
@Injectable()
export class GuildRoleGuard implements CanActivate {
  canActivate(context: ExecutionContext): boolean {
    // Check if user is member/owner of specific guild
    // Implementation in backend/src/guild/guards/guild-role.guard.ts
  }
}
```

## Security Considerations

1. **Never trust client-side role checks** - Always validate on the server
2. **Principle of Least Privilege** - Grant minimum necessary permissions
3. **Audit role assignments** - Log when roles are assigned/changed
4. **Regular reviews** - Periodically review role-based access patterns
5. **Defense in depth** - Combine with other security measures (rate limiting, input validation)

## Summary

The RBAC implementation in Stellar-Guilds provides:

✅ **Simple integration** with existing JWT authentication  
✅ **Declarative syntax** using `@Roles()` decorator  
✅ **Automatic enforcement** via `RoleGuard`  
✅ **Flexible role hierarchy** (USER, MODERATOR, ADMIN, OWNER)  
✅ **Clear error messages** for debugging  
✅ **Easy to extend** for future requirements

All protected endpoints follow the pattern:

```typescript
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(...allowedRoles)
```

This ensures consistent, secure access control across the entire application.
