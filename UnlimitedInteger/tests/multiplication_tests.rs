/*

		a = new UnlimitedInteger("12"); System.out.println("A is " + a);
		b = new UnlimitedInteger("7"); System.out.println("B is " + b);

		c = (a.multiply(b));
		System.out.print("According to you, " + a + " * " + b + " = " + c);
		if (c.toString().equals("84")) System.out.println("  Correct");  else System.out.println("  Wrong!");


		a = new UnlimitedInteger("487652"); System.out.println("A is " + a);
		b = new UnlimitedInteger("365396"); System.out.println("B is " + b);


        c = (a.multiply(b));
		System.out.print("According to you, " + a + " * " + b + " = " + c);
		if (c.toString().equals("178186090192")) System.out.println("  Correct");  else System.out.println("  Wrong!");


		// Multiplication
		System.out.println("------------------ Multiplication ------------------");
		for (int i = 0; i < num_of_probs_to_check; i++) {
			BigInteger x = new BigInteger(num_of_bits_per_number, rnd);
			BigInteger y = new BigInteger(num_of_bits_per_number, rnd);

			a = new UnlimitedInteger(x.toString());
			b = new UnlimitedInteger(y.toString());
			c = a.multiply(b);
			System.out.println(a + "\n x \n" + b + "\n = \n" + c);
			if (c.toString().equals(x.multiply(y).toString())) {
				System.out.println("That is correct.\n");
			} else {
				System.out.println("That is wrong.\n");
			}
		}




 */