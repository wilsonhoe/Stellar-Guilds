import { Injectable } from '@nestjs/common';
import { LoginDto } from './dto/login.dto';
import { RegisterDto } from './dto/register.dto';

@Injectable()
export class AuthenticationService {
  async register(dto: RegisterDto) {
    return {
      message: 'User registered successfully',
      user: {
        email: dto.email,
      },
    };
  }

  async login(dto: LoginDto) {
    return {
      message: 'User logged in successfully',
      user: {
        email: dto.email,
      },
      token: 'mock-jwt-token',
    };
  }
}