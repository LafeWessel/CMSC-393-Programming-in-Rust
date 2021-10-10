/*

		a = new UnlimitedInteger("12"); System.out.println("A is " + a);
		b = new UnlimitedInteger("7"); System.out.println("B is " + b);

		c = (a.pow(2));
		System.out.print("According to you, " + a + " ^ " + 2 + " = " + c);
		if (c.toString().equals("144")) System.out.println("  Correct"); else System.out.println("  Wrong!");

		a = new UnlimitedInteger("487652"); System.out.println("A is " + a);
		b = new UnlimitedInteger("365396"); System.out.println("B is " + b);

	    c = (a.pow(7));
		System.out.print("According to you, " + a + " ^ " + 7 + " = " + c);
		if (c.toString().equals("6557979700838660104333950213065903587328")) System.out.println("  Correct"); else System.out.println("  Wrong!");


		// Exponents
		System.out.println("------------------ Exponents ------------------");
		for (int i = 0; i < num_of_probs_to_check; i++) {
			BigInteger x = new BigInteger(num_of_bits_per_number, rnd);
			int y = rnd.nextInt(10)+5;

			a = new UnlimitedInteger(x.toString());
			c = a.pow(y);
			System.out.println(a + " ^ " + y + " = " + c);
			if (c.toString().equals(x.pow(y).toString())) {
				System.out.println("That is correct.\n");
			} else {
				System.out.println("That is wrong.\n");
			}
		}

 */