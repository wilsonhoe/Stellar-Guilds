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
import { UserRole } from './../user/dto/user.dto';

/**
 * Example Controller demonstrating RBAC usage
 *
 * This shows how to protect endpoints with role-based access control
 * that integrates with the existing JWT authentication system.
 */
@Controller('examples')
export class ExampleController {
  /**
   * PUBLIC ENDPOINT - No authentication required
   * Anyone can access this endpoint
   */
  @Get('public')
  async publicEndpoint() {
    return {
      message: 'This is public - no auth required',
      access: 'everyone',
    };
  }

  /**
   * AUTHENTICATED ENDPOINT - Any logged-in user can access
   * Requires valid JWT token but no specific role
   */
  @Get('authenticated')
  @UseGuards(JwtAuthGuard)
  async authenticatedEndpoint() {
    return {
      message: 'Any authenticated user can access this',
      access: 'all authenticated users',
    };
  }

  /**
   * USER-ONLY ENDPOINT - Basic authenticated users
   * Restricts access to USER role and above
   */
  @Get('user-data')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.USER, UserRole.MODERATOR, UserRole.ADMIN, UserRole.OWNER)
  async userDataEndpoint() {
    return {
      message: 'Basic authenticated users only',
      access: 'USER+',
    };
  }

  /**
   * MODERATOR ENDPOINT - Content moderators
   * Allows MODERATOR, ADMIN, and OWNER roles
   */
  @Get('moderator')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.MODERATOR, UserRole.ADMIN, UserRole.OWNER)
  async moderatorEndpoint() {
    return {
      message: 'Moderators and above can access',
      access: 'MODERATOR+',
    };
  }

  /**
   * ADMIN ENDPOINT - Administrators only
   * Only ADMIN and OWNER roles can access
   */
  @Get('admin')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN, UserRole.OWNER)
  async adminEndpoint() {
    return {
      message: 'Administrators only',
      access: 'ADMIN+',
    };
  }

  /**
   * OWNER ENDPOINT - System owner only
   * Most restrictive - only OWNER role
   */
  @Get('owner')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.OWNER)
  async ownerEndpoint() {
    return {
      message: 'System owner only',
      access: 'OWNER ONLY',
    };
  }

  /**
   * CREATE RESOURCE - Example of protecting POST operations
   * Only ADMIN can create new resources
   */
  @Post('admin/create')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN, UserRole.OWNER)
  async createResource(@Body() body: any) {
    return {
      message: 'Resource created successfully',
      data: body,
      access: 'ADMIN+',
    };
  }

  /**
   * DELETE RESOURCE - Example of protecting DELETE operations
   * Only ADMIN or OWNER can delete resources
   */
  @Delete('admin/delete/:id')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN, UserRole.OWNER)
  async deleteResource(@Param('id') id: string) {
    return {
      message: `Resource ${id} deleted`,
      access: 'ADMIN+',
    };
  }

  /**
   * MULTI-ROLE ENDPOINT - Multiple roles with different access levels
   * Shows how to allow multiple roles to access the same endpoint
   */
  @Get('content/manage')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.MODERATOR, UserRole.ADMIN, UserRole.OWNER)
  async manageContent() {
    return {
      message: 'Content management - Moderators and Admins',
      access: 'MODERATOR+',
    };
  }

  /**
   * USER MANAGEMENT - Example from actual codebase
   * Shows real-world usage with multiple role requirements
   */
  @Get('users/by-role/:role')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN, UserRole.MODERATOR)
  async getUsersByRole(@Param('role') role: UserRole) {
    // In real implementation, this would call a service
    return {
      message: `Fetching users with role: ${role}`,
      access: 'ADMIN or MODERATOR only',
    };
  }

  /**
   * ASSIGN ROLE - Highly restricted operation
   * Only ADMIN can assign roles to other users
   */
  @Post('users/:userId/assign-role')
  @UseGuards(JwtAuthGuard, RoleGuard)
  @Roles(UserRole.ADMIN)
  async assignRole(
    @Param('userId') userId: string,
    @Body() roleData: { role: UserRole },
  ) {
    return {
      message: `Assigned role ${roleData.role} to user ${userId}`,
      access: 'ADMIN ONLY - Sensitive operation',
    };
  }
}

/**
 * TESTING EXAMPLES
 *
 * Test with curl or Postman:
 *
 * 1. Public endpoint (no token needed):
 *    GET http://localhost:3000/examples/public
 *
 * 2. Authenticated endpoint (needs valid JWT):
 *    GET http://localhost:3000/examples/auth
 *    Header: Authorization: Bearer <your-jwt-token>
 *
 * 3. Admin endpoint (needs ADMIN role):
 *    GET http://localhost:3000/examples/admin
 *    Header: Authorization: Bearer <admin-jwt-token>
 *
 * Expected responses:
 * - 200 OK: Access granted
 * - 401 Unauthorized: No token or invalid token
 * - 403 Forbidden: Valid token but insufficient role
 */

/**
 * KEY POINTS:
 *
 * 1. Always use @UseGuards(JwtAuthGuard, RoleGuard) together
 * 2. Order matters: JwtAuthGuard first, then RoleGuard
 * 3. Use @Roles() to specify which roles can access
 * 4. Can specify multiple roles: @Roles(UserRole.ADMIN, UserRole.MODERATOR)
 * 5. Role hierarchy is not automatic - explicitly list all allowed roles
 * 6. The user's role is included in the JWT token payload
 * 7. RoleGuard reads the role from request.user (set by JwtAuthGuard)
 */
