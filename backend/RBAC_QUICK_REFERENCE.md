# RBAC Quick Reference Card

## 🚀 One-Liner Usage

```typescript
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)
```

---

## 📦 Required Imports

```typescript
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RoleGuard } from '../auth/guards/role.guard';
import { Roles } from '../auth/decorators/roles.decorator';
import { UserRole } from '../../user/dto/user.dto';
```

---

## 🎯 Common Patterns

### Admin Only

```typescript
@Get('admin/stats')
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)
async getAdminStats() { ... }
```

### Admin or Moderator

```typescript
@Get('content/review')
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN, UserRole.MODERATOR)
async reviewContent() { ... }
```

### Any Authenticated User

```typescript
@Get('profile')
@UseGuards(JwtAuthGuard)  // No RoleGuard needed
async getProfile() { ... }
```

### Public (No Auth)

```typescript
@Get('public-info')
async getPublicInfo() { ... }  // No guards
```

---

## 👥 Role Levels

| Role      | Use Case                  |
| --------- | ------------------------- |
| USER      | Basic authenticated users |
| MODERATOR | Content moderation        |
| ADMIN     | System administration     |
| OWNER     | Full access               |

---

## ✅ Complete Example

```typescript
import {
  Controller,
  Get,
  Post,
  Delete,
  UseGuards,
  Param,
  Body,
} from '@nestjs/common';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { RoleGuard } from '../auth/guards/role.guard';
import { Roles } from '../auth/decorators/roles.decorator';
import { UserRole } from '../../user/dto/user.dto';

@Controller('users')
export class UserController {
  // Public - No auth required
  @Get('all')
  async getAllUsers() {
    return this.userService.findAll();
  }

  // Any authenticated user
  @Get('me')
  @UseGuards(JwtAuthGuard)
  async getCurrentUser() {
    return this.userService.findMe();
  }

  // Moderators and above
  @Get('pending-approval')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.MODERATOR, UserRole.ADMIN, UserRole.OWNER)
  async getPendingApprovals() {
    return this.userService.findPending();
  }

  // Admin only
  @Delete(':id')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN)
  async deleteUser(@Param('id') id: string) {
    return this.userService.delete(id);
  }
}
```

---

## 🧪 Testing

### Generate Test Token

```typescript
const token = jwtService.sign({
  sub: 'user-id',
  email: 'user@example.com',
  role: UserRole.ADMIN,
});
```

### Test Protected Endpoint

```bash
curl -H "Authorization: Bearer <TOKEN>" \
  http://localhost:3000/api/endpoint
```

---

## ⚠️ Common Mistakes

### ❌ Wrong Order

```typescript
@UseGuards(RoleGuard, JwtAuthGuard)  // WRONG!
```

### ✅ Correct Order

```typescript
@UseGuards(JwtAuthGuard, RoleGuard)  // CORRECT!
```

### ❌ Missing Roles Decorator

```typescript
@UseGuards(JwtAuthGuard, RoleGuard)
@Get('admin')
async adminOnly() { ... }  // Will allow ALL authenticated users!
```

### ✅ Add Roles Decorator

```typescript
@UseGuards(JwtAuthGuard, RoleGuard)
@Roles(UserRole.ADMIN)
@Get('admin')
async adminOnly() { ... }  // Now properly restricted!
```

---

## 🔍 Error Responses

### 401 Unauthorized

```json
{
  "statusCode": 401,
  "message": "Unauthorized"
}
```

**Cause**: No token or invalid token

### 403 Forbidden

```json
{
  "statusCode": 403,
  "message": "User role \"USER\" does not have access to this resource"
}
```

**Cause**: Valid token but insufficient role

---

## 📚 Full Documentation

- **RBAC_GUIDE.md** - Comprehensive guide
- **rbac-example.controller.ts** - Working examples
- **rbac.integration.spec.ts** - Test suite
- **RBAC_IMPLEMENTATION_SUMMARY.md** - Implementation overview

---

## 💡 Pro Tips

1. Always list minimum required roles
2. Document why specific roles are needed
3. Test with different role tokens
4. Use hierarchical role checks explicitly
5. Combine with other guards for fine-grained control

---

**Quick Start**: Copy the pattern at the top and modify for your needs!
